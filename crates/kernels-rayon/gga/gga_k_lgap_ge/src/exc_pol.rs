//! GGA_K_LGAP_GE exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap_ge.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lgap_ge_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_mu_0: f64,
    param_mu_1: f64,
    param_mu_2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu_0 = f64x8::splat(param_mu_0);
    let param_mu_1 = f64x8::splat(param_mu_1);
    let param_mu_2 = f64x8::splat(param_mu_2);
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
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
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
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t33 = f64x8::splat(M_CBRT6);
            let t34 = t33 * t33;
            let t35 = param_mu_0 * t34;
            let t36 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t37 = (simd::cbrt(t36));
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = ((v_sigma0).sqrt());
            let t40 = t38 * t39;
            let t41 = (simd::cbrt(v_rho0));
            let t43 = f64x8::splat(1.0) / t41 / v_rho0;
            let t48 = param_mu_1 * t33;
            let t49 = t37 * t37;
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t50 * v_sigma0;
            let t52 = v_rho0 * v_rho0;
            let t53 = t41 * t41;
            let t55 = f64x8::splat(1.0) / t53 / t52;
            let t61 = param_mu_2 / t36;
            let t62 = t39 * v_sigma0;
            let t63 = t52 * t52;
            let t64 = f64x8::splat(1.0) / t63;
            let t68 = f64x8::splat(1.0) + t35 * t40 * t43 / f64x8::splat(12.0) + t48 * t51 * t55 / f64x8::splat(24.0) + t61 * t62 * t64 / f64x8::splat(48.0);
            let t72 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t68));
            let t73 = (v_rho1).simd_le(dens_threshold);
            let t74 = -t17;
            let t76 = ((t15).select(t12, (t11).select(t16, t74 * t8)));
            let t77 = f64x8::splat(1.0) + t76;
            let t78 = (t77).simd_le(zeta_threshold);
            let t79 = (simd::cbrt(t77));
            let t80 = t79 * t79;
            let t82 = ((t78).select(t24, t80 * t77));
            let t83 = t82 * t30;
            let t84 = ((v_sigma2).sqrt());
            let t85 = t38 * t84;
            let t86 = (simd::cbrt(v_rho1));
            let t88 = f64x8::splat(1.0) / t86 / v_rho1;
            let t92 = t50 * v_sigma2;
            let t93 = v_rho1 * v_rho1;
            let t94 = t86 * t86;
            let t96 = f64x8::splat(1.0) / t94 / t93;
            let t100 = t84 * v_sigma2;
            let t101 = t93 * t93;
            let t102 = f64x8::splat(1.0) / t101;
            let t106 = f64x8::splat(1.0) + t35 * t85 * t88 / f64x8::splat(12.0) + t48 * t92 * t96 / f64x8::splat(24.0) + t61 * t100 * t102 / f64x8::splat(48.0);
            let t110 = ((t73).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t83 * t106));
            let tzk0 = t72 + t110;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
