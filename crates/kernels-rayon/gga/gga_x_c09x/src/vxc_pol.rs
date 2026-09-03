//! GGA_X_C09X vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_c09x.c`
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
pub fn gga_x_c09x_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t39 = v_sigma0 * t38;
            let t40 = t33 * t39;
            let t42 = (simd::exp(-f64x8::splat(0.0020125) * t40));
            let t47 = (simd::exp(-f64x8::splat(0.00100625) * t40));
            let t49 = f64x8::splat(2.245) + f64x8::splat(0.0025708333333333334) * t33 * t39 * t42 - f64x8::splat(1.245) * t47;
            let t53 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t49));
            let t54 = (v_rho1).simd_le(dens_threshold);
            let t55 = -t16;
            let t57 = ((t14).select(t11, (t10).select(t15, t55 * t7)));
            let t58 = f64x8::splat(1.0) + t57;
            let t59 = (t58).simd_le(zeta_threshold);
            let t60 = (simd::cbrt(t58));
            let t62 = ((t59).select(t22, t60 * t58));
            let t63 = t62 * t26;
            let t64 = v_rho1 * v_rho1;
            let t65 = (simd::cbrt(v_rho1));
            let t66 = t65 * t65;
            let t68 = f64x8::splat(1.0) / t66 / t64;
            let t69 = v_sigma2 * t68;
            let t70 = t33 * t69;
            let t72 = (simd::exp(-f64x8::splat(0.0020125) * t70));
            let t77 = (simd::exp(-f64x8::splat(0.00100625) * t70));
            let t79 = f64x8::splat(2.245) + f64x8::splat(0.0025708333333333334) * t33 * t69 * t72 - f64x8::splat(1.245) * t77;
            let t83 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t79));
            let tzk0 = t53 + t83;
            acc_zk = tzk0;
            let t84 = t6 * t6;
            let t85 = f64x8::splat(1.0) / t84;
            let t86 = t16 * t85;
            let t88 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t86)));
            let t91 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t88));
            let t92 = t91 * t26;
            let t96 = t26 * t26;
            let t97 = f64x8::splat(1.0) / t96;
            let t98 = t25 * t97;
            let t101 = t5 * t98 * t49 / f64x8::splat(8.0);
            let t102 = t34 * v_rho0;
            let t104 = f64x8::splat(1.0) / t36 / t102;
            let t105 = v_sigma0 * t104;
            let t109 = t28 * t28;
            let t112 = t109 / t30 / t29;
            let t113 = v_sigma0 * v_sigma0;
            let t114 = t34 * t34;
            let t115 = t114 * t34;
            let t117 = f64x8::splat(1.0) / t35 / t115;
            let t125 = -f64x8::splat(0.006855555555555556) * t33 * t105 * t42 + f64x8::splat(1.3796805555555556e-05) * t112 * t113 * t117 * t42 - f64x8::splat(0.00334075) * t33 * t105 * t47;
            let t130 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t92 * t49 - t101 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t125));
            let t131 = t55 * t85;
            let t133 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t131)));
            let t136 = ((t59).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t133));
            let t137 = t136 * t26;
            let t141 = t62 * t97;
            let t144 = t5 * t141 * t79 / f64x8::splat(8.0);
            let t146 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t137 * t79 - t144));
            let tvrho0 = t53 + t83 + t6 * (t130 + t146);
            acc_vrho_0 = tvrho0;
            let t150 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t86)));
            let t153 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t150));
            let t154 = t153 * t26;
            let t159 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t154 * t49 - t101));
            let t161 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t131)));
            let t164 = ((t59).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t60 * t161));
            let t165 = t164 * t26;
            let t169 = t64 * v_rho1;
            let t171 = f64x8::splat(1.0) / t66 / t169;
            let t172 = v_sigma2 * t171;
            let t176 = v_sigma2 * v_sigma2;
            let t177 = t64 * t64;
            let t178 = t177 * t64;
            let t180 = f64x8::splat(1.0) / t65 / t178;
            let t188 = -f64x8::splat(0.006855555555555556) * t33 * t172 * t72 + f64x8::splat(1.3796805555555556e-05) * t112 * t176 * t180 * t72 - f64x8::splat(0.00334075) * t33 * t172 * t77;
            let t193 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t165 * t79 - t144 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t188));
            let tvrho1 = t53 + t83 + t6 * (t159 + t193);
            acc_vrho_1 = tvrho1;
            let t199 = t114 * v_rho0;
            let t201 = f64x8::splat(1.0) / t35 / t199;
            let t209 = f64x8::splat(0.0025708333333333334) * t33 * t38 * t42 - f64x8::splat(5.173802083333333e-06) * t112 * v_sigma0 * t201 * t42 + f64x8::splat(0.00125278125) * t33 * t38 * t47;
            let t213 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t209));
            let tvsigma0 = t6 * t213;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t217 = t177 * v_rho1;
            let t219 = f64x8::splat(1.0) / t65 / t217;
            let t227 = f64x8::splat(0.0025708333333333334) * t33 * t68 * t72 - f64x8::splat(5.173802083333333e-06) * t112 * v_sigma2 * t219 * t72 + f64x8::splat(0.00125278125) * t33 * t68 * t77;
            let t231 = ((t54).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t63 * t227));
            let tvsigma2 = t6 * t231;
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}
