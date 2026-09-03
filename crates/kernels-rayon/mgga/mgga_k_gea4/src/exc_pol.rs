//! MGGA_K_GEA4 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea4.c`
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
pub fn mgga_k_gea4_exc_pol(
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
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = v_rho0 + v_rho1;
            let t9 = f64x8::splat(1.0) / t8;
            let t12 = (f64x8::splat(2.0) * v_rho0 * t9).simd_le(zeta_threshold);
            let t13 = zeta_threshold - f64x8::splat(1.0);
            let t16 = (f64x8::splat(2.0) * v_rho1 * t9).simd_le(zeta_threshold);
            let t17 = -t13;
            let t18 = v_rho0 - v_rho1;
            let t20 = ((t12).select(t13, (t16).select(t17, t18 * t9)));
            let t21 = f64x8::splat(1.0) + t20;
            let t22 = (t21).simd_le(zeta_threshold);
            let t23 = (simd::cbrt(zeta_threshold));
            let t24 = t23 * t23;
            let t25 = t24 * zeta_threshold;
            let t26 = (simd::cbrt(t21));
            let t27 = t26 * t26;
            let t29 = ((t22).select(t25, t27 * t21));
            let t30 = (simd::cbrt(t8));
            let t31 = t30 * t30;
            let t32 = t29 * t31;
            let t33 = f64x8::splat(M_CBRT6);
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t38 = t33 / t36;
            let t39 = v_rho0 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t40 * t40;
            let t43 = f64x8::splat(1.0) / t41 / t39;
            let t48 = f64x8::splat(1.0) / t41 / v_rho0;
            let t52 = t33 * t33;
            let t54 = f64x8::splat(1.0) / t35 / t34;
            let t55 = t52 * t54;
            let t56 = v_lapl0 * v_lapl0;
            let t57 = t39 * v_rho0;
            let t59 = f64x8::splat(1.0) / t40 / t57;
            let t63 = t39 * t39;
            let t65 = f64x8::splat(1.0) / t40 / t63;
            let t66 = v_sigma0 * t65;
            let t70 = v_sigma0 * v_sigma0;
            let t71 = t63 * v_rho0;
            let t73 = f64x8::splat(1.0) / t40 / t71;
            let t77 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t38 * v_sigma0 * t43 + f64x8::splat(5.0) / f64x8::splat(54.0) * t38 * v_lapl0 * t48 + t55 * t56 * t59 / f64x8::splat(5832.0) - t55 * t66 * v_lapl0 / f64x8::splat(5184.0) + t55 * t70 * t73 / f64x8::splat(17496.0);
            let t81 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t77));
            let t82 = (v_rho1).simd_le(dens_threshold);
            let t83 = -t18;
            let t85 = ((t16).select(t13, (t12).select(t17, t83 * t9)));
            let t86 = f64x8::splat(1.0) + t85;
            let t87 = (t86).simd_le(zeta_threshold);
            let t88 = (simd::cbrt(t86));
            let t89 = t88 * t88;
            let t91 = ((t87).select(t25, t89 * t86));
            let t92 = t91 * t31;
            let t93 = v_rho1 * v_rho1;
            let t94 = (simd::cbrt(v_rho1));
            let t95 = t94 * t94;
            let t97 = f64x8::splat(1.0) / t95 / t93;
            let t102 = f64x8::splat(1.0) / t95 / v_rho1;
            let t106 = v_lapl1 * v_lapl1;
            let t107 = t93 * v_rho1;
            let t109 = f64x8::splat(1.0) / t94 / t107;
            let t113 = t93 * t93;
            let t115 = f64x8::splat(1.0) / t94 / t113;
            let t116 = v_sigma2 * t115;
            let t120 = v_sigma2 * v_sigma2;
            let t121 = t113 * v_rho1;
            let t123 = f64x8::splat(1.0) / t94 / t121;
            let t127 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t38 * v_sigma2 * t97 + f64x8::splat(5.0) / f64x8::splat(54.0) * t38 * v_lapl1 * t102 + t55 * t106 * t109 / f64x8::splat(5832.0) - t55 * t116 * v_lapl1 / f64x8::splat(5184.0) + t55 * t120 * t123 / f64x8::splat(17496.0);
            let t131 = ((t82).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t92 * t127));
            let tzk0 = t81 + t131;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}
