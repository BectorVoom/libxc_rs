//! GGA_K_APBE kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbe.c`
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
pub fn gga_k_apbe_kxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
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
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
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
            let t32 = f64x8::splat(M_CBRT6);
            let t33 = param_mu * t32;
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t37 * v_sigma0;
            let t39 = v_rho0 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t40 * t40;
            let t43 = f64x8::splat(1.0) / t41 / t39;
            let t47 = param_kappa + t33 * t38 * t43 / f64x8::splat(24.0);
            let t52 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t47);
            let t56 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t52));
            let t57 = (v_rho1).simd_le(dens_threshold);
            let t58 = -t17;
            let t60 = ((t15).select(t12, (t11).select(t16, t58 * t8)));
            let t61 = f64x8::splat(1.0) + t60;
            let t62 = (t61).simd_le(zeta_threshold);
            let t63 = (simd::cbrt(t61));
            let t64 = t63 * t63;
            let t66 = ((t62).select(t24, t64 * t61));
            let t67 = t66 * t30;
            let t68 = t37 * v_sigma2;
            let t69 = v_rho1 * v_rho1;
            let t70 = (simd::cbrt(v_rho1));
            let t71 = t70 * t70;
            let t73 = f64x8::splat(1.0) / t71 / t69;
            let t77 = param_kappa + t33 * t68 * t73 / f64x8::splat(24.0);
            let t82 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t77);
            let t86 = ((t57).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t67 * t82));
            let tzk0 = t56 + t86;
            acc_zk = tzk0;
            let t87 = t7 * t7;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = t17 * t88;
            let t91 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t89)));
            let t94 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t91));
            let t95 = t94 * t30;
            let t99 = f64x8::splat(1.0) / t29;
            let t100 = t28 * t99;
            let t103 = t6 * t100 * t52 / f64x8::splat(10.0);
            let t104 = param_kappa * param_kappa;
            let t105 = t31 * t104;
            let t106 = t6 * t105;
            let t107 = t47 * t47;
            let t109 = f64x8::splat(1.0) / t107 * param_mu;
            let t110 = t109 * t32;
            let t111 = t39 * v_rho0;
            let t113 = f64x8::splat(1.0) / t41 / t111;
            let t115 = t110 * t38 * t113;
            let t119 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t95 * t52 + t103 - t106 * t115 / f64x8::splat(60.0)));
            let t120 = t58 * t88;
            let t122 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t120)));
            let t125 = ((t62).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t64 * t122));
            let t126 = t125 * t30;
            let t130 = t66 * t99;
            let t133 = t6 * t130 * t82 / f64x8::splat(10.0);
            let t135 = ((t57).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t126 * t82 + t133));
            let tvrho0 = t56 + t86 + t7 * (t119 + t135);
            acc_vrho_0 = tvrho0;
            let t139 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t89)));
            let t142 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t139));
            let t143 = t142 * t30;
            let t148 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t143 * t52 + t103));
            let t150 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t120)));
            let t153 = ((t62).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t64 * t150));
            let t154 = t153 * t30;
            let t158 = t67 * t104;
            let t159 = t6 * t158;
            let t160 = t77 * t77;
            let t162 = f64x8::splat(1.0) / t160 * param_mu;
            let t163 = t162 * t32;
            let t164 = t69 * v_rho1;
            let t166 = f64x8::splat(1.0) / t71 / t164;
            let t168 = t163 * t68 * t166;
            let t172 = ((t57).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t154 * t82 + t133 - t159 * t168 / f64x8::splat(60.0)));
            let tvrho1 = t56 + t86 + t7 * (t148 + t172);
            acc_vrho_1 = tvrho1;
            let t175 = t32 * t37;
            let t177 = t109 * t175 * t43;
            let t180 = ((t1).select(f64x8::splat(0.0), t106 * t177 / f64x8::splat(160.0)));
            let tvsigma0 = t7 * t180;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t182 = t162 * t175 * t73;
            let t185 = ((t57).select(f64x8::splat(0.0), t159 * t182 / f64x8::splat(160.0)));
            let tvsigma2 = t7 * t185;
            acc_vsigma_2 = tvsigma2;
            let t188 = f64x8::splat(1.0) / t25;
            let t189 = t91 * t91;
            let t192 = t87 * t7;
            let t193 = f64x8::splat(1.0) / t192;
            let t194 = t17 * t193;
            let t197 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t88 + f64x8::splat(2.0) * t194)));
            let t201 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t188 * t189 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t197));
            let t202 = t201 * t30;
            let t206 = t94 * t99;
            let t208 = t6 * t206 * t52;
            let t211 = t6 * t95 * t104;
            let t215 = f64x8::splat(1.0) / t29 / t7;
            let t216 = t28 * t215;
            let t219 = t6 * t216 * t52 / f64x8::splat(30.0);
            let t221 = t6 * t100 * t104;
            let t222 = t221 * t115;
            let t226 = param_mu * param_mu;
            let t227 = f64x8::splat(1.0) / t107 / t47 * t226;
            let t228 = t32 * t32;
            let t229 = t227 * t228;
            let t231 = f64x8::splat(1.0) / t35 / t34;
            let t232 = v_sigma0 * v_sigma0;
            let t233 = t231 * t232;
            let t234 = t39 * t39;
            let t237 = f64x8::splat(1.0) / t40 / t234 / t111;
            let t239 = t229 * t233 * t237;
            let t243 = f64x8::splat(1.0) / t41 / t234;
            let t245 = t110 * t38 * t243;
            let t249 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t202 * t52 + t208 / f64x8::splat(5.0) - t211 * t115 / f64x8::splat(30.0) - t219 - t222 / f64x8::splat(45.0) - t106 * t239 / f64x8::splat(270.0) + f64x8::splat(11.0) / f64x8::splat(180.0) * t106 * t245));
            let t250 = f64x8::splat(1.0) / t63;
            let t251 = t122 * t122;
            let t254 = t58 * t193;
            let t257 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t88 + f64x8::splat(2.0) * t254)));
            let t261 = ((t62).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t250 * t251 + f64x8::splat(5.0) / f64x8::splat(3.0) * t64 * t257));
            let t262 = t261 * t30;
            let t266 = t125 * t99;
            let t268 = t6 * t266 * t82;
            let t270 = t66 * t215;
            let t273 = t6 * t270 * t82 / f64x8::splat(30.0);
            let t275 = ((t57).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t262 * t82 + t268 / f64x8::splat(5.0) - t273));
            let tv2rho20 = f64x8::splat(2.0) * t119 + f64x8::splat(2.0) * t135 + t7 * (t249 + t275);
            acc_v2rho2_0 = tv2rho20;
            let t278 = t188 * t139;
            let t282 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t194)));
            let t286 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t278 * t91 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t282));
            let t287 = t286 * t30;
            let t291 = t142 * t99;
            let t293 = t6 * t291 * t52;
            let t296 = t6 * t143 * t104;
            let t302 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t287 * t52 + t293 / f64x8::splat(10.0) - t296 * t115 / f64x8::splat(60.0) + t208 / f64x8::splat(10.0) - t219 - t222 / f64x8::splat(90.0)));
            let t303 = t250 * t150;
            let t307 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t254)));
            let t311 = ((t62).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t303 * t122 + f64x8::splat(5.0) / f64x8::splat(3.0) * t64 * t307));
            let t312 = t311 * t30;
            let t316 = t153 * t99;
            let t318 = t6 * t316 * t82;
            let t322 = t6 * t126 * t104;
            let t326 = t6 * t130 * t104;
            let t327 = t326 * t168;
            let t330 = ((t57).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t312 * t82 + t318 / f64x8::splat(10.0) + t268 / f64x8::splat(10.0) - t273 - t322 * t168 / f64x8::splat(60.0) - t327 / f64x8::splat(90.0)));
            let tv2rho21 = t119 + t135 + t148 + t172 + t7 * (t302 + t330);
            acc_v2rho2_1 = tv2rho21;
            let t335 = t139 * t139;
            let t340 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(2.0) * t88 + f64x8::splat(2.0) * t194)));
            let t344 = ((t21).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t188 * t335 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t340));
            let t345 = t344 * t30;
            let t351 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t345 * t52 + t293 / f64x8::splat(5.0) - t219));
            let t352 = t150 * t150;
            let t357 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t88 + f64x8::splat(2.0) * t254)));
            let t361 = ((t62).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t250 * t352 + f64x8::splat(5.0) / f64x8::splat(3.0) * t64 * t357));
            let t362 = t361 * t30;
            let t368 = t6 * t154 * t104;
            let t374 = f64x8::splat(1.0) / t160 / t77 * t226;
            let t375 = t374 * t228;
            let t376 = v_sigma2 * v_sigma2;
            let t377 = t231 * t376;
            let t378 = t69 * t69;
            let t381 = f64x8::splat(1.0) / t70 / t378 / t164;
            let t383 = t375 * t377 * t381;
            let t387 = f64x8::splat(1.0) / t71 / t378;
            let t389 = t163 * t68 * t387;
            let t393 = ((t57).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t362 * t82 + t318 / f64x8::splat(5.0) - t368 * t168 / f64x8::splat(30.0) - t273 - t327 / f64x8::splat(45.0) - t159 * t383 / f64x8::splat(270.0) + f64x8::splat(11.0) / f64x8::splat(180.0) * t159 * t389));
            let tv2rho22 = f64x8::splat(2.0) * t148 + f64x8::splat(2.0) * t172 + t7 * (t351 + t393);
            acc_v2rho2_2 = tv2rho22;
            let t399 = t221 * t177 / f64x8::splat(240.0);
            let t400 = t234 * t39;
            let t402 = f64x8::splat(1.0) / t40 / t400;
            let t405 = t229 * t231 * t402 * v_sigma0;
            let t409 = t109 * t175 * t113;
            let t413 = ((t1).select(f64x8::splat(0.0), t211 * t177 / f64x8::splat(160.0) + t399 + t106 * t405 / f64x8::splat(720.0) - t106 * t409 / f64x8::splat(60.0)));
            let tv2rhosigma0 = t7 * t413 + t180;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t418 = t326 * t182 / f64x8::splat(240.0);
            let t420 = ((t57).select(f64x8::splat(0.0), t322 * t182 / f64x8::splat(160.0) + t418));
            let tv2rhosigma2 = t7 * t420 + t185;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t425 = ((t1).select(f64x8::splat(0.0), t296 * t177 / f64x8::splat(160.0) + t399));
            let tv2rhosigma3 = t7 * t425 + t180;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t429 = t378 * t69;
            let t431 = f64x8::splat(1.0) / t70 / t429;
            let t434 = t375 * t231 * t431 * v_sigma2;
            let t438 = t162 * t175 * t166;
            let t442 = ((t57).select(f64x8::splat(0.0), t368 * t182 / f64x8::splat(160.0) + t418 + t159 * t434 / f64x8::splat(720.0) - t159 * t438 / f64x8::splat(60.0)));
            let tv2rhosigma5 = t7 * t442 + t185;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t444 = t228 * t231;
            let t445 = t234 * v_rho0;
            let t449 = t227 * t444 / t40 / t445;
            let t452 = ((t1).select(f64x8::splat(0.0), -t106 * t449 / f64x8::splat(1920.0)));
            let tv2sigma20 = t7 * t452;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t453 = t378 * v_rho1;
            let t457 = t374 * t444 / t70 / t453;
            let t460 = ((t57).select(f64x8::splat(0.0), -t159 * t457 / f64x8::splat(1920.0)));
            let tv2sigma25 = t7 * t460;
            acc_v2sigma2_5 = tv2sigma25;
            let t464 = f64x8::splat(1.0) / t25 / t20;
            let t465 = t189 * t91;
            let t468 = t188 * t91;
            let t471 = t87 * t87;
            let t472 = f64x8::splat(1.0) / t471;
            let t473 = t17 * t472;
            let t476 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), f64x8::splat(6.0) * t193 - f64x8::splat(6.0) * t473)));
            let t480 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t464 * t465 + f64x8::splat(10.0) / f64x8::splat(3.0) * t468 * t197 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t476));
            let t481 = t480 * t30;
            let t485 = t234 * t234;
            let t487 = f64x8::splat(1.0) / t40 / t485;
            let t489 = t229 * t233 * t487;
            let t493 = f64x8::splat(1.0) / t41 / t445;
            let t495 = t110 * t38 * t493;
            let t500 = t221 * t245;
            let t505 = t6 * t216 * t104;
            let t506 = t505 * t115;
            let t508 = t221 * t239;
            let t511 = t6 * t202 * t104;
            let t515 = t6 * t206 * t104;
            let t516 = t515 * t115;
            let t518 = t201 * t99;
            let t520 = t6 * t518 * t52;
            let t522 = t94 * t215;
            let t524 = t6 * t522 * t52;
            let t527 = f64x8::splat(1.0) / t29 / t87;
            let t528 = t28 * t527;
            let t531 = f64x8::splat(2.0) / f64x8::splat(45.0) * t6 * t528 * t52;
            let t532 = t4 * t4;
            let t535 = t3 / t532 / t34;
            let t536 = t535 * t31;
            let t537 = t107 * t107;
            let t539 = t104 / t537;
            let t540 = t226 * param_mu;
            let t541 = t232 * v_sigma0;
            let t542 = t540 * t541;
            let t543 = t485 * t111;
            let t544 = f64x8::splat(1.0) / t543;
            let t546 = t539 * t542 * t544;
            let t549 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t481 * t52 + f64x8::splat(11.0) / f64x8::splat(270.0) * t106 * t489 - f64x8::splat(77.0) / f64x8::splat(270.0) * t106 * t495 + f64x8::splat(11.0) / f64x8::splat(60.0) * t211 * t245 + f64x8::splat(11.0) / f64x8::splat(90.0) * t500 - t211 * t239 / f64x8::splat(90.0) + t506 / f64x8::splat(90.0) - t508 / f64x8::splat(135.0) - t511 * t115 / f64x8::splat(20.0) - t516 / f64x8::splat(15.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t520 - t524 / f64x8::splat(10.0) + t531 - t536 * t546 / f64x8::splat(135.0);
            let t550 = ((t1).select(f64x8::splat(0.0), t549));
            let t552 = f64x8::splat(1.0) / t63 / t61;
            let t553 = t251 * t122;
            let t556 = t250 * t122;
            let t559 = t58 * t472;
            let t562 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t193 - f64x8::splat(6.0) * t559)));
            let t566 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t552 * t553 + f64x8::splat(10.0) / f64x8::splat(3.0) * t556 * t257 + f64x8::splat(5.0) / f64x8::splat(3.0) * t64 * t562));
            let t567 = t566 * t30;
            let t571 = t261 * t99;
            let t573 = t6 * t571 * t82;
            let t575 = t125 * t215;
            let t577 = t6 * t575 * t82;
            let t579 = t66 * t527;
            let t582 = f64x8::splat(2.0) / f64x8::splat(45.0) * t6 * t579 * t82;
            let t584 = ((t57).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t567 * t82 + f64x8::splat(3.0) / f64x8::splat(10.0) * t573 - t577 / f64x8::splat(10.0) + t582));
            let tv3rho30 = f64x8::splat(3.0) * t249 + f64x8::splat(3.0) * t275 + t7 * (t550 + t584);
            acc_v3rho3_0 = tv3rho30;
            let t587 = f64x8::splat(2.0) * t302;
            let t588 = f64x8::splat(2.0) * t330;
            let t589 = t464 * t139;
            let t592 = t188 * t282;
            let t597 = f64x8::splat(2.0) * t193;
            let t598 = f64x8::splat(6.0) * t473;
            let t600 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t597 - t598)));
            let t604 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t589 * t189 + f64x8::splat(20.0) / f64x8::splat(9.0) * t592 * t91 + f64x8::splat(10.0) / f64x8::splat(9.0) * t278 * t197 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t600));
            let t605 = t604 * t30;
            let t609 = t286 * t99;
            let t612 = t6 * t609 * t52 / f64x8::splat(5.0);
            let t614 = t6 * t287 * t104;
            let t617 = t142 * t215;
            let t619 = t6 * t617 * t52;
            let t622 = t6 * t291 * t104;
            let t624 = t622 * t115 / f64x8::splat(45.0);
            let t635 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t605 * t52 + t612 - t614 * t115 / f64x8::splat(30.0) - t619 / f64x8::splat(30.0) - t624 - t296 * t239 / f64x8::splat(270.0) + f64x8::splat(11.0) / f64x8::splat(180.0) * t296 * t245 + t520 / f64x8::splat(10.0) - t524 / f64x8::splat(15.0) - t516 / f64x8::splat(45.0) + t531 + t506 / f64x8::splat(135.0) - t508 / f64x8::splat(405.0) + f64x8::splat(11.0) / f64x8::splat(270.0) * t500;
            let t636 = ((t1).select(f64x8::splat(0.0), t635));
            let t637 = t552 * t150;
            let t640 = t250 * t307;
            let t645 = f64x8::splat(6.0) * t559;
            let t647 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t597 - t645)));
            let t651 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t637 * t251 + f64x8::splat(20.0) / f64x8::splat(9.0) * t640 * t122 + f64x8::splat(10.0) / f64x8::splat(9.0) * t303 * t257 + f64x8::splat(5.0) / f64x8::splat(3.0) * t64 * t647));
            let t652 = t651 * t30;
            let t656 = t311 * t99;
            let t659 = t6 * t656 * t82 / f64x8::splat(5.0);
            let t660 = t153 * t215;
            let t662 = t6 * t660 * t82;
            let t667 = t6 * t262 * t104;
            let t671 = t6 * t266 * t104;
            let t673 = t671 * t168 / f64x8::splat(45.0);
            let t675 = t6 * t270 * t104;
            let t676 = t675 * t168;
            let t679 = ((t57).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t652 * t82 + t659 - t662 / f64x8::splat(30.0) + t573 / f64x8::splat(10.0) - t577 / f64x8::splat(15.0) + t582 - t667 * t168 / f64x8::splat(60.0) - t673 + t676 / f64x8::splat(270.0)));
            let tv3rho31 = t249 + t275 + t587 + t588 + t7 * (t636 + t679);
            acc_v3rho3_1 = tv3rho31;
            let t682 = t464 * t335;
            let t687 = t188 * t340;
            let t691 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t597 - t598)));
            let t695 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t682 * t91 + f64x8::splat(20.0) / f64x8::splat(9.0) * t278 * t282 + f64x8::splat(10.0) / f64x8::splat(9.0) * t687 * t91 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t691));
            let t696 = t695 * t30;
            let t700 = t344 * t99;
            let t702 = t6 * t700 * t52;
            let t705 = t6 * t345 * t104;
            let t712 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t696 * t52 + t702 / f64x8::splat(10.0) - t705 * t115 / f64x8::splat(60.0) + t612 - t619 / f64x8::splat(15.0) - t624 - t524 / f64x8::splat(30.0) + t531 + t506 / f64x8::splat(270.0)));
            let t713 = t552 * t352;
            let t718 = t250 * t357;
            let t722 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t597 - t645)));
            let t726 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t713 * t122 + f64x8::splat(20.0) / f64x8::splat(9.0) * t303 * t307 + f64x8::splat(10.0) / f64x8::splat(9.0) * t718 * t122 + f64x8::splat(5.0) / f64x8::splat(3.0) * t64 * t722));
            let t727 = t726 * t30;
            let t731 = t361 * t99;
            let t733 = t6 * t731 * t82;
            let t737 = t6 * t312 * t104;
            let t741 = t6 * t316 * t104;
            let t742 = t741 * t168;
            let t748 = t326 * t383;
            let t752 = t326 * t389;
            let t754 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t727 * t82 + t733 / f64x8::splat(10.0) + t659 - t662 / f64x8::splat(15.0) - t737 * t168 / f64x8::splat(30.0) - t742 / f64x8::splat(45.0) - t577 / f64x8::splat(30.0) + t582 - t673 + t676 / f64x8::splat(135.0) - t322 * t383 / f64x8::splat(270.0) - t748 / f64x8::splat(405.0) + f64x8::splat(11.0) / f64x8::splat(180.0) * t322 * t389 + f64x8::splat(11.0) / f64x8::splat(270.0) * t752;
            let t755 = ((t57).select(f64x8::splat(0.0), t754));
            let tv3rho32 = t587 + t588 + t351 + t393 + t7 * (t712 + t755);
            acc_v3rho3_2 = tv3rho32;
            let t760 = t335 * t139;
            let t767 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t193 - f64x8::splat(6.0) * t473)));
            let t771 = ((t21).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t464 * t760 + f64x8::splat(10.0) / f64x8::splat(3.0) * t278 * t340 + f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t767));
            let t772 = t771 * t30;
            let t779 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t772 * t52 + f64x8::splat(3.0) / f64x8::splat(10.0) * t702 - t619 / f64x8::splat(10.0) + t531));
            let t780 = t352 * t150;
            let t787 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), f64x8::splat(6.0) * t193 - f64x8::splat(6.0) * t559)));
            let t791 = ((t62).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t552 * t780 + f64x8::splat(10.0) / f64x8::splat(3.0) * t303 * t357 + f64x8::splat(5.0) / f64x8::splat(3.0) * t64 * t787));
            let t792 = t791 * t30;
            let t797 = f64x8::splat(1.0) / t71 / t453;
            let t799 = t163 * t68 * t797;
            let t802 = t378 * t378;
            let t804 = f64x8::splat(1.0) / t70 / t802;
            let t806 = t375 * t377 * t804;
            let t818 = t6 * t362 * t104;
            let t823 = t535 * t67;
            let t824 = t160 * t160;
            let t826 = t104 / t824;
            let t827 = t376 * v_sigma2;
            let t828 = t540 * t827;
            let t829 = t802 * t164;
            let t830 = f64x8::splat(1.0) / t829;
            let t832 = t826 * t828 * t830;
            let t835 = f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t792 * t82 - f64x8::splat(77.0) / f64x8::splat(270.0) * t159 * t799 + f64x8::splat(11.0) / f64x8::splat(270.0) * t159 * t806 + f64x8::splat(11.0) / f64x8::splat(90.0) * t752 + f64x8::splat(11.0) / f64x8::splat(60.0) * t368 * t389 - t742 / f64x8::splat(15.0) - t748 / f64x8::splat(135.0) + t676 / f64x8::splat(90.0) - t368 * t383 / f64x8::splat(90.0) - t818 * t168 / f64x8::splat(20.0) + f64x8::splat(3.0) / f64x8::splat(10.0) * t733 - t662 / f64x8::splat(10.0) + t582 - t823 * t832 / f64x8::splat(135.0);
            let t836 = ((t57).select(f64x8::splat(0.0), t835));
            let tv3rho33 = f64x8::splat(3.0) * t351 + f64x8::splat(3.0) * t393 + t7 * (t779 + t836);
            acc_v3rho3_3 = tv3rho33;
            let t842 = t515 * t177;
            let t849 = t505 * t177 / f64x8::splat(720.0);
            let t850 = t221 * t405;
            let t852 = t221 * t409;
            let t854 = t485 * t39;
            let t856 = t540 / t854;
            let t858 = t539 * t856 * t232;
            let t863 = t229 * t231 * t237 * v_sigma0;
            let t867 = t109 * t175 * t243;
            let t871 = ((t1).select(f64x8::splat(0.0), t511 * t177 / f64x8::splat(160.0) + t842 / f64x8::splat(120.0) + t211 * t405 / f64x8::splat(360.0) - t211 * t409 / f64x8::splat(30.0) - t849 + t850 / f64x8::splat(540.0) - t852 / f64x8::splat(45.0) + t536 * t858 / f64x8::splat(360.0) - t106 * t863 / f64x8::splat(80.0) + f64x8::splat(11.0) / f64x8::splat(180.0) * t106 * t867));
            let tv3rho2sigma0 = t7 * t871 + f64x8::splat(2.0) * t413;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t876 = t671 * t182;
            let t879 = t675 * t182 / f64x8::splat(720.0);
            let t881 = ((t57).select(f64x8::splat(0.0), t667 * t182 / f64x8::splat(160.0) + t876 / f64x8::splat(120.0) - t879));
            let tv3rho2sigma2 = t7 * t881 + f64x8::splat(2.0) * t420;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t885 = t622 * t177;
            let t895 = ((t1).select(f64x8::splat(0.0), t614 * t177 / f64x8::splat(160.0) + t885 / f64x8::splat(240.0) + t296 * t405 / f64x8::splat(720.0) - t296 * t409 / f64x8::splat(60.0) + t842 / f64x8::splat(240.0) - t849 + t850 / f64x8::splat(1080.0) - t852 / f64x8::splat(90.0)));
            let tv3rho2sigma3 = t7 * t895 + t413 + t425;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t899 = t741 * t182;
            let t904 = t326 * t434;
            let t908 = t326 * t438;
            let t911 = ((t57).select(f64x8::splat(0.0), t737 * t182 / f64x8::splat(160.0) + t899 / f64x8::splat(240.0) + t876 / f64x8::splat(240.0) - t879 + t322 * t434 / f64x8::splat(720.0) + t904 / f64x8::splat(1080.0) - t322 * t438 / f64x8::splat(60.0) - t908 / f64x8::splat(90.0)));
            let tv3rho2sigma5 = t7 * t911 + t420 + t442;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t918 = ((t1).select(f64x8::splat(0.0), t705 * t177 / f64x8::splat(160.0) + t885 / f64x8::splat(120.0) - t849));
            let tv3rho2sigma6 = t7 * t918 + f64x8::splat(2.0) * t425;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t930 = t802 * t69;
            let t932 = t540 / t930;
            let t934 = t826 * t932 * t376;
            let t939 = t375 * t231 * t381 * v_sigma2;
            let t943 = t162 * t175 * t387;
            let t947 = ((t57).select(f64x8::splat(0.0), t818 * t182 / f64x8::splat(160.0) + t899 / f64x8::splat(120.0) + t368 * t434 / f64x8::splat(360.0) - t368 * t438 / f64x8::splat(30.0) - t879 + t904 / f64x8::splat(540.0) - t908 / f64x8::splat(45.0) + t823 * t934 / f64x8::splat(360.0) - t159 * t939 / f64x8::splat(80.0) + f64x8::splat(11.0) / f64x8::splat(180.0) * t159 * t943));
            let tv3rho2sigma8 = t7 * t947 + f64x8::splat(2.0) * t442;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t952 = t221 * t449 / f64x8::splat(2880.0);
            let t953 = t485 * v_rho0;
            let t955 = t540 / t953;
            let t957 = t539 * t955 * v_sigma0;
            let t961 = t227 * t444 * t402;
            let t965 = ((t1).select(f64x8::splat(0.0), -t211 * t449 / f64x8::splat(1920.0) - t952 - t536 * t957 / f64x8::splat(960.0) + t106 * t961 / f64x8::splat(360.0)));
            let tv3rhosigma20 = t7 * t965 + t452;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t970 = t326 * t457 / f64x8::splat(2880.0);
            let t972 = ((t57).select(f64x8::splat(0.0), -t322 * t457 / f64x8::splat(1920.0) - t970));
            let tv3rhosigma25 = t7 * t972 + t460;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t977 = ((t1).select(f64x8::splat(0.0), -t296 * t449 / f64x8::splat(1920.0) - t952));
            let tv3rhosigma26 = t7 * t977 + t452;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t981 = t802 * v_rho1;
            let t983 = t540 / t981;
            let t985 = t826 * t983 * v_sigma2;
            let t989 = t374 * t444 * t431;
            let t993 = ((t57).select(f64x8::splat(0.0), -t368 * t457 / f64x8::splat(1920.0) - t970 - t823 * t985 / f64x8::splat(960.0) + t159 * t989 / f64x8::splat(360.0)));
            let tv3rhosigma211 = t7 * t993 + t460;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t997 = t539 * t540 / t485;
            let t1000 = ((t1).select(f64x8::splat(0.0), t536 * t997 / f64x8::splat(2560.0)));
            let tv3sigma30 = t7 * t1000;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1003 = t826 * t540 / t802;
            let t1006 = ((t57).select(f64x8::splat(0.0), t823 * t1003 / f64x8::splat(2560.0)));
            let tv3sigma39 = t7 * t1006;
            acc_v3sigma3_9 = tv3sigma39;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}
