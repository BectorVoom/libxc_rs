//! GGA_XC_TH1 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th1.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_xc_th1_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_20: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    param_omega_18: f64,
    param_omega_19: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_omega_0 = f64x8::splat(param_omega_0);
    let param_omega_1 = f64x8::splat(param_omega_1);
    let param_omega_2 = f64x8::splat(param_omega_2);
    let param_omega_3 = f64x8::splat(param_omega_3);
    let param_omega_4 = f64x8::splat(param_omega_4);
    let param_omega_5 = f64x8::splat(param_omega_5);
    let param_omega_6 = f64x8::splat(param_omega_6);
    let param_omega_7 = f64x8::splat(param_omega_7);
    let param_omega_8 = f64x8::splat(param_omega_8);
    let param_omega_9 = f64x8::splat(param_omega_9);
    let param_omega_10 = f64x8::splat(param_omega_10);
    let param_omega_11 = f64x8::splat(param_omega_11);
    let param_omega_12 = f64x8::splat(param_omega_12);
    let param_omega_13 = f64x8::splat(param_omega_13);
    let param_omega_14 = f64x8::splat(param_omega_14);
    let param_omega_15 = f64x8::splat(param_omega_15);
    let param_omega_20 = f64x8::splat(param_omega_20);
    let param_omega_16 = f64x8::splat(param_omega_16);
    let param_omega_17 = f64x8::splat(param_omega_17);
    let param_omega_18 = f64x8::splat(param_omega_18);
    let param_omega_19 = f64x8::splat(param_omega_19);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t1 = param_omega_0;
            let t2 = (simd::pow(v_rho0, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t3 = t2 * v_rho0;
            let t4 = (simd::pow(v_rho1, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t5 = t4 * v_rho1;
            let t6 = t3 + t5;
            let t8 = param_omega_1;
            let t9 = (simd::cbrt(v_rho0));
            let t10 = t9 * v_rho0;
            let t11 = (simd::cbrt(v_rho1));
            let t12 = t11 * v_rho1;
            let t13 = t10 + t12;
            let t15 = param_omega_2;
            let t16 = ((v_rho0).sqrt());
            let t17 = t16 * v_rho0;
            let t18 = ((v_rho1).sqrt());
            let t19 = t18 * v_rho1;
            let t20 = t17 + t19;
            let t22 = param_omega_3;
            let t23 = t9 * t9;
            let t24 = t23 * v_rho0;
            let t25 = t11 * t11;
            let t26 = t25 * v_rho1;
            let t27 = t24 + t26;
            let t29 = param_omega_4;
            let t30 = t29 * t13;
            let t31 = ((v_sigma0).sqrt());
            let t32 = f64x8::splat(1.0) / t10;
            let t33 = t31 * t32;
            let t34 = v_rho0 - v_rho1;
            let t35 = v_rho0 + v_rho1;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t34 * t36;
            let t38 = f64x8::splat(1.0) + t37;
            let t39 = (t38).simd_le(zeta_threshold);
            let t40 = (simd::cbrt(zeta_threshold));
            let t41 = t40 * zeta_threshold;
            let t42 = (simd::cbrt(t38));
            let t44 = ((t39).select(t41, t42 * t38));
            let t45 = f64x8::splat(M_CBRT2);
            let t46 = t45 * t45;
            let t47 = t44 * t46;
            let t49 = ((v_sigma2).sqrt());
            let t50 = f64x8::splat(1.0) / t12;
            let t51 = t49 * t50;
            let t52 = f64x8::splat(1.0) - t37;
            let t53 = (t52).simd_le(zeta_threshold);
            let t54 = (simd::cbrt(t52));
            let t56 = ((t53).select(t41, t54 * t52));
            let t57 = t56 * t46;
            let t60 = t33 * t47 / f64x8::splat(4.0) + t51 * t57 / f64x8::splat(4.0);
            let t63 = param_omega_5;
            let t64 = t63 * t20;
            let t67 = param_omega_6;
            let t68 = t67 * t27;
            let t71 = param_omega_7;
            let t72 = t2 * t2;
            let t73 = t72 * t72;
            let t74 = t73 * t2;
            let t75 = t74 * v_rho0;
            let t76 = t4 * t4;
            let t77 = t76 * t76;
            let t78 = t77 * t4;
            let t79 = t78 * v_rho1;
            let t80 = t75 + t79;
            let t81 = t71 * t80;
            let t84 = param_omega_8;
            let t85 = t84 * t20;
            let t86 = v_rho0 * v_rho0;
            let t88 = f64x8::splat(1.0) / t23 / t86;
            let t89 = v_sigma0 * t88;
            let t90 = t44 * t44;
            let t91 = t90 * t45;
            let t92 = t89 * t91;
            let t93 = v_rho1 * v_rho1;
            let t95 = f64x8::splat(1.0) / t25 / t93;
            let t96 = v_sigma2 * t95;
            let t97 = t56 * t56;
            let t98 = t97 * t45;
            let t99 = t96 * t98;
            let t101 = t92 / f64x8::splat(8.0) + t99 / f64x8::splat(8.0);
            let t104 = param_omega_9;
            let t105 = t104 * t27;
            let t109 = param_omega_10;
            let t110 = t109 * t80;
            let t113 = param_omega_11;
            let t114 = t86 + t93;
            let t115 = t113 * t114;
            let t118 = param_omega_12;
            let t119 = t118 * t20;
            let t123 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t124 = t35 * t35;
            let t125 = (simd::cbrt(t35));
            let t126 = t125 * t125;
            let t128 = f64x8::splat(1.0) / t126 / t124;
            let t130 = t92 / f64x8::splat(4.0) + t99 / f64x8::splat(4.0) - t123 * t128;
            let t132 = param_omega_13;
            let t133 = t132 * t27;
            let t135 = param_omega_14;
            let t136 = t135 * t80;
            let t138 = param_omega_15;
            let t139 = t138 * t114;
            let t141 = param_omega_16;
            let t142 = t141 * t6;
            let t143 = t34 * t34;
            let t144 = f64x8::splat(1.0) / t124;
            let t145 = t143 * t144;
            let t147 = param_omega_17;
            let t148 = t147 * t13;
            let t150 = param_omega_18;
            let t151 = t150 * t20;
            let t153 = param_omega_19;
            let t154 = t153 * t27;
            let t156 = param_omega_20;
            let t158 = t110 * t101 / f64x8::splat(2.0) + t115 * t101 / f64x8::splat(2.0) + t119 * t130 + t133 * t130 + t136 * t130 + t139 * t130 + t142 * t145 + t148 * t145 + t151 * t145 + t154 * t145 + t156 * t35;
            let tzk0 = (t1 * t6 + t8 * t13 + t15 * t20 + t22 * t27 + t30 * t60 / f64x8::splat(2.0) + t64 * t60 / f64x8::splat(2.0) + t68 * t60 / f64x8::splat(2.0) + t81 * t60 / f64x8::splat(2.0) + t85 * t101 / f64x8::splat(2.0) + t105 * t101 / f64x8::splat(2.0) + t158) * t36;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
