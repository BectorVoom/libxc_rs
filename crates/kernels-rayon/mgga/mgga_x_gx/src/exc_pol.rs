//! MGGA_X_GX exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gx.c`
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
pub fn mgga_x_gx_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c0: f64,
    param_c1: f64,
    param_alphainf: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c0 = f64x8::splat(param_c0);
    let param_c1 = f64x8::splat(param_c1);
    let param_alphainf = f64x8::splat(param_alphainf);
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t3 * t3;
            let t32 = f64x8::splat(M_CBRT4);
            let t34 = f64x8::splat(8.0) / f64x8::splat(27.0) * t29 * t30 * t32;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / v_rho0;
            let t40 = v_rho0 * v_rho0;
            let t42 = f64x8::splat(1.0) / t36 / t40;
            let t45 = v_tau0 * t38 - v_sigma0 * t42 / f64x8::splat(8.0);
            let t46 = f64x8::splat(M_CBRT6);
            let t48 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t49 = (simd::cbrt(t48));
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t45 * t46 * t51;
            let t54 = t46 * t51;
            let t57 = param_c0 + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c1 * t45 * t54;
            let t58 = param_c0 + param_c1 - f64x8::splat(1.0);
            let t62 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * t58 * t45 * t54;
            let t63 = f64x8::splat(1.0) / t62;
            let t65 = f64x8::splat(1.0) - t34;
            let t66 = t57 * t63 * t65;
            let t69 = t34 + f64x8::splat(5.0) / f64x8::splat(9.0) * t52 * t66;
            let t70 = f64x8::splat(5.0) / f64x8::splat(9.0) * t52;
            let t71 = f64x8::splat(1.0) - t70;
            let t72 = ((t71).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t74 = f64x8::splat(1.0) - param_alphainf;
            let t75 = t74 * t71;
            let t76 = f64x8::splat(1.0) + t70;
            let t77 = f64x8::splat(1.0) / t76;
            let t79 = t75 * t77 + f64x8::splat(1.0);
            let t80 = -t71;
            let t81 = ((t80).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t83 = t69 * t72 + t79 * t81;
            let t87 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t83));
            let t88 = (v_rho1).simd_le(dens_threshold);
            let t89 = -t17;
            let t91 = ((t15).select(t12, (t11).select(t16, t89 * t8)));
            let t92 = f64x8::splat(1.0) + t91;
            let t93 = (t92).simd_le(zeta_threshold);
            let t94 = (simd::cbrt(t92));
            let t96 = ((t93).select(t23, t94 * t92));
            let t97 = t96 * t27;
            let t98 = (simd::cbrt(v_rho1));
            let t99 = t98 * t98;
            let t101 = f64x8::splat(1.0) / t99 / v_rho1;
            let t103 = v_rho1 * v_rho1;
            let t105 = f64x8::splat(1.0) / t99 / t103;
            let t108 = v_tau1 * t101 - v_sigma2 * t105 / f64x8::splat(8.0);
            let t110 = t108 * t46 * t51;
            let t114 = param_c0 + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c1 * t108 * t54;
            let t118 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * t58 * t108 * t54;
            let t119 = f64x8::splat(1.0) / t118;
            let t121 = t114 * t119 * t65;
            let t124 = t34 + f64x8::splat(5.0) / f64x8::splat(9.0) * t110 * t121;
            let t125 = f64x8::splat(5.0) / f64x8::splat(9.0) * t110;
            let t126 = f64x8::splat(1.0) - t125;
            let t127 = ((t126).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t129 = t74 * t126;
            let t130 = f64x8::splat(1.0) + t125;
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = t129 * t131 + f64x8::splat(1.0);
            let t134 = -t126;
            let t135 = ((t134).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t137 = t124 * t127 + t133 * t135;
            let t141 = ((t88).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t97 * t137));
            let tzk0 = t87 + t141;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
