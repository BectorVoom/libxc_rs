//! MGGA_X_PBE_GX exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pbe_gx.c`
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
pub fn mgga_x_pbe_gx_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t3 * t3;
            let t32 = f64x8::splat(M_CBRT4);
            let t34 = f64x8::splat(8.0) / f64x8::splat(27.0) * t29 * t30 * t32;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / v_rho0;
            let t40 = v_rho0 * v_rho0;
            let t42 = f64x8::splat(1.0) / t36 / t40;
            let t43 = v_sigma0 * t42;
            let t45 = v_tau0 * t38 - t43 / f64x8::splat(8.0);
            let t46 = f64x8::splat(M_CBRT6);
            let t48 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t49 = (simd::cbrt(t48));
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t45 * t46 * t51;
            let t54 = f64x8::splat(0.827411) - f64x8::splat(0.3575333333333333) * t52;
            let t56 = f64x8::splat(1.0) - f64x8::splat(0.45341611111111113) * t52;
            let t57 = f64x8::splat(1.0) / t56;
            let t59 = f64x8::splat(1.0) - t34;
            let t60 = t54 * t57 * t59;
            let t63 = t34 + f64x8::splat(5.0) / f64x8::splat(9.0) * t52 * t60;
            let t64 = f64x8::splat(5.0) / f64x8::splat(9.0) * t52;
            let t65 = f64x8::splat(1.0) - t64;
            let t66 = ((t65).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t68 = f64x8::splat(1.0) + t64;
            let t69 = f64x8::splat(1.0) / t68;
            let t72 = f64x8::splat(1.0) + f64x8::splat(0.148) * t65 * t69;
            let t73 = -t65;
            let t74 = ((t73).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t76 = t63 * t66 + t72 * t74;
            let t79 = f64x8::splat(1.0) + f64x8::splat(0.001015549) * t43;
            let t80 = f64x8::splat(1.0) / t79;
            let t81 = t28 * t76 * t80;
            let t84 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t81));
            let t85 = (v_rho1).simd_le(dens_threshold);
            let t86 = -t17;
            let t88 = ((t15).select(t12, (t11).select(t16, t86 * t8)));
            let t89 = f64x8::splat(1.0) + t88;
            let t90 = (t89).simd_le(zeta_threshold);
            let t91 = (simd::cbrt(t89));
            let t93 = ((t90).select(t23, t91 * t89));
            let t94 = t6 * t93;
            let t95 = (simd::cbrt(v_rho1));
            let t96 = t95 * t95;
            let t98 = f64x8::splat(1.0) / t96 / v_rho1;
            let t100 = v_rho1 * v_rho1;
            let t102 = f64x8::splat(1.0) / t96 / t100;
            let t103 = v_sigma2 * t102;
            let t105 = v_tau1 * t98 - t103 / f64x8::splat(8.0);
            let t107 = t105 * t46 * t51;
            let t109 = f64x8::splat(0.827411) - f64x8::splat(0.3575333333333333) * t107;
            let t111 = f64x8::splat(1.0) - f64x8::splat(0.45341611111111113) * t107;
            let t112 = f64x8::splat(1.0) / t111;
            let t114 = t109 * t112 * t59;
            let t117 = t34 + f64x8::splat(5.0) / f64x8::splat(9.0) * t107 * t114;
            let t118 = f64x8::splat(5.0) / f64x8::splat(9.0) * t107;
            let t119 = f64x8::splat(1.0) - t118;
            let t120 = ((t119).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t122 = f64x8::splat(1.0) + t118;
            let t123 = f64x8::splat(1.0) / t122;
            let t126 = f64x8::splat(1.0) + f64x8::splat(0.148) * t119 * t123;
            let t127 = -t119;
            let t128 = ((t127).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t130 = t117 * t120 + t126 * t128;
            let t133 = f64x8::splat(1.0) + f64x8::splat(0.001015549) * t103;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t28 * t130 * t134;
            let t138 = ((t85).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t94 * t135));
            let tzk0 = t84 + t138;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}
