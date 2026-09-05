//! GGA_X_RPBE kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rpbe.c`
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
pub fn gga_x_rpbe_kxc_pol(
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
    param_rpbe_mu: f64,
    param_rpbe_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_rpbe_mu = f64x8::splat(param_rpbe_mu);
    let param_rpbe_kappa = f64x8::splat(param_rpbe_kappa);
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
            let t29 = param_rpbe_mu * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t41 = f64x8::splat(1.0) / param_rpbe_kappa;
            let t45 = (simd::exp(-t34 * v_sigma0 * t39 * t41 / f64x8::splat(24.0)));
            let t48 = f64x8::splat(1.0) + param_rpbe_kappa * (f64x8::splat(1.0) - t45);
            let t52 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t48));
            let t53 = (v_rho1).simd_le(dens_threshold);
            let t54 = -t16;
            let t56 = ((t14).select(t11, (t10).select(t15, t54 * t7)));
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(t57));
            let t61 = ((t58).select(t22, t59 * t57));
            let t62 = t61 * t26;
            let t63 = v_rho1 * v_rho1;
            let t64 = (simd::cbrt(v_rho1));
            let t65 = t64 * t64;
            let t67 = f64x8::splat(1.0) / t65 / t63;
            let t72 = (simd::exp(-t34 * v_sigma2 * t67 * t41 / f64x8::splat(24.0)));
            let t75 = f64x8::splat(1.0) + param_rpbe_kappa * (f64x8::splat(1.0) - t72);
            let t79 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t62 * t75));
            let tzk0 = t52 + t79;
            acc_zk = tzk0;
            let t80 = t6 * t6;
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t16 * t81;
            let t84 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t82)));
            let t87 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t84));
            let t88 = t87 * t26;
            let t92 = t26 * t26;
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t25 * t93;
            let t97 = t5 * t94 * t48 / f64x8::splat(8.0);
            let t99 = t5 * t27 * param_rpbe_mu;
            let t100 = t28 * t33;
            let t101 = t35 * v_rho0;
            let t103 = f64x8::splat(1.0) / t37 / t101;
            let t106 = t100 * v_sigma0 * t103 * t45;
            let t110 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t88 * t48 - t97 + t99 * t106 / f64x8::splat(24.0)));
            let t111 = t54 * t81;
            let t113 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t111)));
            let t116 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t113));
            let t117 = t116 * t26;
            let t121 = t61 * t93;
            let t124 = t5 * t121 * t75 / f64x8::splat(8.0);
            let t126 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t117 * t75 - t124));
            let tvrho0 = t52 + t79 + t6 * (t110 + t126);
            acc_vrho_0 = tvrho0;
            let t130 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t82)));
            let t133 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t130));
            let t134 = t133 * t26;
            let t139 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t134 * t48 - t97));
            let t141 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t111)));
            let t144 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t141));
            let t145 = t144 * t26;
            let t150 = t5 * t62 * param_rpbe_mu;
            let t151 = t63 * v_rho1;
            let t153 = f64x8::splat(1.0) / t65 / t151;
            let t156 = t100 * v_sigma2 * t153 * t72;
            let t160 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t145 * t75 - t124 + t150 * t156 / f64x8::splat(24.0)));
            let tvrho1 = t52 + t79 + t6 * (t139 + t160);
            acc_vrho_1 = tvrho1;
            let t163 = t5 * t27;
            let t166 = t29 * t33 * t39 * t45;
            let t169 = ((t1).select(f64x8::splat(0.0), -t163 * t166 / f64x8::splat(64.0)));
            let tvsigma0 = t6 * t169;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t170 = t5 * t62;
            let t173 = t29 * t33 * t67 * t72;
            let t176 = ((t53).select(f64x8::splat(0.0), -t170 * t173 / f64x8::splat(64.0)));
            let tvsigma2 = t6 * t176;
            acc_vsigma_2 = tvsigma2;
            let t179 = t23 * t23;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t84 * t84;
            let t184 = t80 * t6;
            let t185 = f64x8::splat(1.0) / t184;
            let t186 = t16 * t185;
            let t189 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t186)));
            let t193 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t180 * t181 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t189));
            let t194 = t193 * t26;
            let t198 = t87 * t93;
            let t200 = t5 * t198 * t48;
            let t203 = t5 * t88 * param_rpbe_mu;
            let t207 = f64x8::splat(1.0) / t92 / t6;
            let t208 = t25 * t207;
            let t211 = t5 * t208 * t48 / f64x8::splat(12.0);
            let t213 = t5 * t94 * param_rpbe_mu;
            let t214 = t213 * t106;
            let t216 = t35 * t35;
            let t218 = f64x8::splat(1.0) / t37 / t216;
            let t221 = t100 * v_sigma0 * t218 * t45;
            let t224 = param_rpbe_mu * param_rpbe_mu;
            let t226 = t5 * t27 * t224;
            let t227 = t28 * t28;
            let t230 = t227 / t31 / t30;
            let t231 = v_sigma0 * v_sigma0;
            let t232 = t230 * t231;
            let t235 = f64x8::splat(1.0) / t36 / t216 / t101;
            let t237 = t235 * t41 * t45;
            let t238 = t232 * t237;
            let t242 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t194 * t48 - t200 / f64x8::splat(4.0) + t203 * t106 / f64x8::splat(12.0) + t211 + t214 / f64x8::splat(36.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t99 * t221 + t226 * t238 / f64x8::splat(216.0)));
            let t243 = t59 * t59;
            let t244 = f64x8::splat(1.0) / t243;
            let t245 = t113 * t113;
            let t248 = t54 * t185;
            let t251 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t248)));
            let t255 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t244 * t245 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t251));
            let t256 = t255 * t26;
            let t260 = t116 * t93;
            let t262 = t5 * t260 * t75;
            let t264 = t61 * t207;
            let t267 = t5 * t264 * t75 / f64x8::splat(12.0);
            let t269 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t256 * t75 - t262 / f64x8::splat(4.0) + t267));
            let tv2rho20 = f64x8::splat(2.0) * t110 + f64x8::splat(2.0) * t126 + t6 * (t242 + t269);
            acc_v2rho2_0 = tv2rho20;
            let t272 = t180 * t130;
            let t276 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t186)));
            let t280 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t272 * t84 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t276));
            let t281 = t280 * t26;
            let t285 = t133 * t93;
            let t287 = t5 * t285 * t48;
            let t290 = t5 * t134 * param_rpbe_mu;
            let t296 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t281 * t48 - t287 / f64x8::splat(8.0) + t290 * t106 / f64x8::splat(24.0) - t200 / f64x8::splat(8.0) + t211 + t214 / f64x8::splat(72.0)));
            let t297 = t244 * t141;
            let t301 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t248)));
            let t305 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t297 * t113 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t301));
            let t306 = t305 * t26;
            let t310 = t144 * t93;
            let t312 = t5 * t310 * t75;
            let t316 = t5 * t117 * param_rpbe_mu;
            let t320 = t5 * t121 * param_rpbe_mu;
            let t321 = t320 * t156;
            let t324 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t306 * t75 - t312 / f64x8::splat(8.0) - t262 / f64x8::splat(8.0) + t267 + t316 * t156 / f64x8::splat(24.0) + t321 / f64x8::splat(72.0)));
            let tv2rho21 = t110 + t126 + t139 + t160 + t6 * (t296 + t324);
            acc_v2rho2_1 = tv2rho21;
            let t329 = t130 * t130;
            let t334 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t186)));
            let t338 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t180 * t329 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t334));
            let t339 = t338 * t26;
            let t345 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t339 * t48 - t287 / f64x8::splat(4.0) + t211));
            let t346 = t141 * t141;
            let t351 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t248)));
            let t355 = ((t58).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t244 * t346 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t351));
            let t356 = t355 * t26;
            let t362 = t5 * t145 * param_rpbe_mu;
            let t366 = t63 * t63;
            let t368 = f64x8::splat(1.0) / t65 / t366;
            let t371 = t100 * v_sigma2 * t368 * t72;
            let t375 = t5 * t62 * t224;
            let t376 = v_sigma2 * v_sigma2;
            let t377 = t230 * t376;
            let t380 = f64x8::splat(1.0) / t64 / t366 / t151;
            let t382 = t380 * t41 * t72;
            let t383 = t377 * t382;
            let t387 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t356 * t75 - t312 / f64x8::splat(4.0) + t362 * t156 / f64x8::splat(12.0) + t267 + t321 / f64x8::splat(36.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t150 * t371 + t375 * t383 / f64x8::splat(216.0)));
            let tv2rho22 = f64x8::splat(2.0) * t139 + f64x8::splat(2.0) * t160 + t6 * (t345 + t387);
            acc_v2rho2_2 = tv2rho22;
            let t390 = t5 * t88;
            let t393 = t5 * t94;
            let t395 = t393 * t166 / f64x8::splat(192.0);
            let t398 = t29 * t33 * t103 * t45;
            let t401 = t216 * t35;
            let t403 = f64x8::splat(1.0) / t36 / t401;
            let t406 = v_sigma0 * t41 * t45;
            let t407 = t230 * t403 * t406;
            let t411 = ((t1).select(f64x8::splat(0.0), -t390 * t166 / f64x8::splat(64.0) - t395 + t163 * t398 / f64x8::splat(24.0) - t226 * t407 / f64x8::splat(576.0)));
            let tv2rhosigma0 = t6 * t411 + t169;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t413 = t5 * t117;
            let t416 = t5 * t121;
            let t418 = t416 * t173 / f64x8::splat(192.0);
            let t420 = ((t53).select(f64x8::splat(0.0), -t413 * t173 / f64x8::splat(64.0) - t418));
            let tv2rhosigma2 = t6 * t420 + t176;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t422 = t5 * t134;
            let t426 = ((t1).select(f64x8::splat(0.0), -t422 * t166 / f64x8::splat(64.0) - t395));
            let tv2rhosigma3 = t6 * t426 + t169;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t428 = t5 * t145;
            let t433 = t29 * t33 * t153 * t72;
            let t436 = t366 * t63;
            let t438 = f64x8::splat(1.0) / t64 / t436;
            let t441 = v_sigma2 * t41 * t72;
            let t442 = t230 * t438 * t441;
            let t446 = ((t53).select(f64x8::splat(0.0), -t428 * t173 / f64x8::splat(64.0) - t418 + t170 * t433 / f64x8::splat(24.0) - t375 * t442 / f64x8::splat(576.0)));
            let tv2rhosigma5 = t6 * t446 + t176;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t448 = t216 * v_rho0;
            let t453 = t230 / t36 / t448 * t41 * t45;
            let t456 = ((t1).select(f64x8::splat(0.0), t226 * t453 / f64x8::splat(1536.0)));
            let tv2sigma20 = t6 * t456;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t457 = t366 * v_rho1;
            let t462 = t230 / t64 / t457 * t41 * t72;
            let t465 = ((t53).select(f64x8::splat(0.0), t375 * t462 / f64x8::splat(1536.0)));
            let tv2sigma25 = t6 * t465;
            acc_v2sigma2_5 = tv2sigma25;
            let t469 = t5 * t194 * param_rpbe_mu;
            let t473 = t5 * t198 * param_rpbe_mu;
            let t474 = t473 * t106;
            let t477 = t5 * t208 * param_rpbe_mu;
            let t478 = t477 * t106;
            let t481 = f64x8::splat(1.0) / t37 / t448;
            let t484 = t100 * v_sigma0 * t481 * t45;
            let t487 = t216 * t216;
            let t489 = f64x8::splat(1.0) / t36 / t487;
            let t492 = t232 * t489 * t41 * t45;
            let t498 = t5 * t88 * t224;
            let t501 = t213 * t221;
            let t504 = t5 * t94 * t224;
            let t505 = t504 * t238;
            let t507 = t193 * t93;
            let t509 = t5 * t507 * t48;
            let t511 = t87 * t207;
            let t513 = t5 * t511 * t48;
            let t516 = f64x8::splat(1.0) / t92 / t80;
            let t517 = t25 * t516;
            let t520 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t517 * t48;
            let t522 = f64x8::splat(1.0) / t179 / t19;
            let t523 = t181 * t84;
            let t526 = t180 * t84;
            let t529 = t80 * t80;
            let t530 = f64x8::splat(1.0) / t529;
            let t531 = t16 * t530;
            let t534 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t185 - f64x8::splat(6.0) * t531)));
            let t538 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t522 * t523 + f64x8::splat(4.0) / f64x8::splat(3.0) * t526 * t189 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t534));
            let t539 = t538 * t26;
            let t543 = t30 * t30;
            let t546 = t2 / t3 / t543;
            let t547 = t546 * t27;
            let t548 = t224 * param_rpbe_mu;
            let t549 = t231 * v_sigma0;
            let t550 = t548 * t549;
            let t551 = t487 * t101;
            let t552 = f64x8::splat(1.0) / t551;
            let t553 = param_rpbe_kappa * param_rpbe_kappa;
            let t554 = f64x8::splat(1.0) / t553;
            let t557 = t550 * t552 * t554 * t45;
            let t560 = t469 * t106 / f64x8::splat(8.0) + t474 / f64x8::splat(12.0) - t478 / f64x8::splat(36.0) + f64x8::splat(77.0) / f64x8::splat(108.0) * t99 * t484 - f64x8::splat(11.0) / f64x8::splat(216.0) * t226 * t492 - f64x8::splat(11.0) / f64x8::splat(24.0) * t203 * t221 + t498 * t238 / f64x8::splat(72.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t501 + t505 / f64x8::splat(216.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t509 + t513 / f64x8::splat(4.0) - t520 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t539 * t48 + t547 * t557 / f64x8::splat(324.0);
            let t561 = ((t1).select(f64x8::splat(0.0), t560));
            let t563 = f64x8::splat(1.0) / t243 / t57;
            let t564 = t245 * t113;
            let t567 = t244 * t113;
            let t570 = t54 * t530;
            let t573 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t185 - f64x8::splat(6.0) * t570)));
            let t577 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t563 * t564 + f64x8::splat(4.0) / f64x8::splat(3.0) * t567 * t251 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t573));
            let t578 = t577 * t26;
            let t582 = t255 * t93;
            let t584 = t5 * t582 * t75;
            let t586 = t116 * t207;
            let t588 = t5 * t586 * t75;
            let t590 = t61 * t516;
            let t593 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t590 * t75;
            let t595 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t578 * t75 - f64x8::splat(3.0) / f64x8::splat(8.0) * t584 + t588 / f64x8::splat(4.0) - t593));
            let tv3rho30 = f64x8::splat(3.0) * t242 + f64x8::splat(3.0) * t269 + t6 * (t561 + t595);
            acc_v3rho3_0 = tv3rho30;
            let t598 = f64x8::splat(2.0) * t296;
            let t599 = f64x8::splat(2.0) * t324;
            let t600 = t522 * t130;
            let t603 = t180 * t276;
            let t608 = f64x8::splat(2.0) * t185;
            let t609 = f64x8::splat(6.0) * t531;
            let t611 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t608 - t609)));
            let t615 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t600 * t181 + f64x8::splat(8.0) / f64x8::splat(9.0) * t603 * t84 + f64x8::splat(4.0) / f64x8::splat(9.0) * t272 * t189 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t611));
            let t616 = t615 * t26;
            let t620 = t280 * t93;
            let t623 = t5 * t620 * t48 / f64x8::splat(4.0);
            let t625 = t5 * t281 * param_rpbe_mu;
            let t628 = t133 * t207;
            let t630 = t5 * t628 * t48;
            let t633 = t5 * t285 * param_rpbe_mu;
            let t635 = t633 * t106 / f64x8::splat(36.0);
            let t639 = t5 * t134 * t224;
            let t648 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t616 * t48 - t623 + t625 * t106 / f64x8::splat(12.0) + t630 / f64x8::splat(12.0) + t635 - f64x8::splat(11.0) / f64x8::splat(72.0) * t290 * t221 + t639 * t238 / f64x8::splat(216.0) - t509 / f64x8::splat(8.0) + t513 / f64x8::splat(6.0) + t474 / f64x8::splat(36.0) - t520 - t478 / f64x8::splat(54.0) - f64x8::splat(11.0) / f64x8::splat(216.0) * t501 + t505 / f64x8::splat(648.0);
            let t649 = ((t1).select(f64x8::splat(0.0), t648));
            let t650 = t563 * t141;
            let t653 = t244 * t301;
            let t658 = f64x8::splat(6.0) * t570;
            let t660 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t608 - t658)));
            let t664 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t650 * t245 + f64x8::splat(8.0) / f64x8::splat(9.0) * t653 * t113 + f64x8::splat(4.0) / f64x8::splat(9.0) * t297 * t251 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t660));
            let t665 = t664 * t26;
            let t669 = t305 * t93;
            let t672 = t5 * t669 * t75 / f64x8::splat(4.0);
            let t673 = t144 * t207;
            let t675 = t5 * t673 * t75;
            let t680 = t5 * t256 * param_rpbe_mu;
            let t684 = t5 * t260 * param_rpbe_mu;
            let t686 = t684 * t156 / f64x8::splat(36.0);
            let t688 = t5 * t264 * param_rpbe_mu;
            let t689 = t688 * t156;
            let t692 = ((t53).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t665 * t75 - t672 + t675 / f64x8::splat(12.0) - t584 / f64x8::splat(8.0) + t588 / f64x8::splat(6.0) - t593 + t680 * t156 / f64x8::splat(24.0) + t686 - t689 / f64x8::splat(108.0)));
            let tv3rho31 = t242 + t269 + t598 + t599 + t6 * (t649 + t692);
            acc_v3rho3_1 = tv3rho31;
            let t695 = t522 * t329;
            let t700 = t180 * t334;
            let t704 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t608 - t609)));
            let t708 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t695 * t84 + f64x8::splat(8.0) / f64x8::splat(9.0) * t272 * t276 + f64x8::splat(4.0) / f64x8::splat(9.0) * t700 * t84 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t704));
            let t709 = t708 * t26;
            let t713 = t338 * t93;
            let t715 = t5 * t713 * t48;
            let t718 = t5 * t339 * param_rpbe_mu;
            let t725 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t709 * t48 - t715 / f64x8::splat(8.0) + t718 * t106 / f64x8::splat(24.0) - t623 + t630 / f64x8::splat(6.0) + t635 + t513 / f64x8::splat(12.0) - t520 - t478 / f64x8::splat(108.0)));
            let t726 = t563 * t346;
            let t731 = t244 * t351;
            let t735 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t608 - t658)));
            let t739 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t726 * t113 + f64x8::splat(8.0) / f64x8::splat(9.0) * t297 * t301 + f64x8::splat(4.0) / f64x8::splat(9.0) * t731 * t113 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t735));
            let t740 = t739 * t26;
            let t744 = t355 * t93;
            let t746 = t5 * t744 * t75;
            let t750 = t5 * t306 * param_rpbe_mu;
            let t754 = t5 * t310 * param_rpbe_mu;
            let t755 = t754 * t156;
            let t761 = t320 * t371;
            let t764 = t5 * t117 * t224;
            let t768 = t5 * t121 * t224;
            let t769 = t768 * t383;
            let t771 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t740 * t75 - t746 / f64x8::splat(8.0) - t672 + t675 / f64x8::splat(6.0) + t750 * t156 / f64x8::splat(12.0) + t755 / f64x8::splat(36.0) + t588 / f64x8::splat(12.0) - t593 + t686 - t689 / f64x8::splat(54.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t316 * t371 - f64x8::splat(11.0) / f64x8::splat(216.0) * t761 + t764 * t383 / f64x8::splat(216.0) + t769 / f64x8::splat(648.0);
            let t772 = ((t53).select(f64x8::splat(0.0), t771));
            let tv3rho32 = t598 + t599 + t345 + t387 + t6 * (t725 + t772);
            acc_v3rho3_2 = tv3rho32;
            let t777 = t329 * t130;
            let t784 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t185 - f64x8::splat(6.0) * t531)));
            let t788 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t522 * t777 + f64x8::splat(4.0) / f64x8::splat(3.0) * t272 * t334 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t784));
            let t789 = t788 * t26;
            let t796 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t789 * t48 - f64x8::splat(3.0) / f64x8::splat(8.0) * t715 + t630 / f64x8::splat(4.0) - t520));
            let t797 = t346 * t141;
            let t804 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t185 - f64x8::splat(6.0) * t570)));
            let t808 = ((t58).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t563 * t797 + f64x8::splat(4.0) / f64x8::splat(3.0) * t297 * t351 + f64x8::splat(4.0) / f64x8::splat(3.0) * t59 * t804));
            let t809 = t808 * t26;
            let t815 = t5 * t356 * param_rpbe_mu;
            let t819 = f64x8::splat(1.0) / t65 / t457;
            let t822 = t100 * v_sigma2 * t819 * t72;
            let t825 = t366 * t366;
            let t827 = f64x8::splat(1.0) / t64 / t825;
            let t830 = t377 * t827 * t41 * t72;
            let t837 = t5 * t145 * t224;
            let t844 = t546 * t62;
            let t845 = t376 * v_sigma2;
            let t846 = t548 * t845;
            let t847 = t825 * t151;
            let t848 = f64x8::splat(1.0) / t847;
            let t851 = t846 * t848 * t554 * t72;
            let t854 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t809 * t75 + t769 / f64x8::splat(216.0) + t815 * t156 / f64x8::splat(8.0) + f64x8::splat(77.0) / f64x8::splat(108.0) * t150 * t822 - f64x8::splat(11.0) / f64x8::splat(216.0) * t375 * t830 - f64x8::splat(11.0) / f64x8::splat(72.0) * t761 - f64x8::splat(11.0) / f64x8::splat(24.0) * t362 * t371 + t837 * t383 / f64x8::splat(72.0) - t689 / f64x8::splat(36.0) + t755 / f64x8::splat(12.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t746 + t675 / f64x8::splat(4.0) - t593 + t844 * t851 / f64x8::splat(324.0);
            let t855 = ((t53).select(f64x8::splat(0.0), t854));
            let tv3rho33 = f64x8::splat(3.0) * t345 + f64x8::splat(3.0) * t387 + t6 * (t796 + t855);
            acc_v3rho3_3 = tv3rho33;
            let t859 = t5 * t194;
            let t862 = t5 * t198;
            let t863 = t862 * t166;
            let t869 = t5 * t208;
            let t871 = t869 * t166 / f64x8::splat(288.0);
            let t872 = t393 * t398;
            let t874 = t504 * t407;
            let t878 = t29 * t33 * t218 * t45;
            let t882 = t230 * t235 * t406;
            let t885 = t487 * t35;
            let t887 = t548 / t885;
            let t889 = t231 * t554 * t45;
            let t890 = t887 * t889;
            let t894 = ((t1).select(f64x8::splat(0.0), -t859 * t166 / f64x8::splat(64.0) - t863 / f64x8::splat(96.0) + t390 * t398 / f64x8::splat(12.0) - t498 * t407 / f64x8::splat(288.0) + t871 + t872 / f64x8::splat(36.0) - t874 / f64x8::splat(864.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t163 * t878 + t226 * t882 / f64x8::splat(64.0) - t547 * t890 / f64x8::splat(864.0)));
            let tv3rho2sigma0 = t6 * t894 + f64x8::splat(2.0) * t411;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t897 = t5 * t256;
            let t900 = t5 * t260;
            let t901 = t900 * t173;
            let t903 = t5 * t264;
            let t905 = t903 * t173 / f64x8::splat(288.0);
            let t907 = ((t53).select(f64x8::splat(0.0), -t897 * t173 / f64x8::splat(64.0) - t901 / f64x8::splat(96.0) + t905));
            let tv3rho2sigma2 = t6 * t907 + f64x8::splat(2.0) * t420;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t909 = t5 * t281;
            let t912 = t5 * t285;
            let t913 = t912 * t166;
            let t923 = ((t1).select(f64x8::splat(0.0), -t909 * t166 / f64x8::splat(64.0) - t913 / f64x8::splat(192.0) + t422 * t398 / f64x8::splat(24.0) - t639 * t407 / f64x8::splat(576.0) - t863 / f64x8::splat(192.0) + t871 + t872 / f64x8::splat(72.0) - t874 / f64x8::splat(1728.0)));
            let tv3rho2sigma3 = t6 * t923 + t411 + t426;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t925 = t5 * t306;
            let t928 = t5 * t310;
            let t929 = t928 * t173;
            let t934 = t416 * t433;
            let t938 = t768 * t442;
            let t941 = ((t53).select(f64x8::splat(0.0), -t925 * t173 / f64x8::splat(64.0) - t929 / f64x8::splat(192.0) - t901 / f64x8::splat(192.0) + t905 + t413 * t433 / f64x8::splat(24.0) + t934 / f64x8::splat(72.0) - t764 * t442 / f64x8::splat(576.0) - t938 / f64x8::splat(1728.0)));
            let tv3rho2sigma5 = t6 * t941 + t420 + t446;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t944 = t5 * t339;
            let t949 = ((t1).select(f64x8::splat(0.0), -t944 * t166 / f64x8::splat(64.0) - t913 / f64x8::splat(96.0) + t871));
            let tv3rho2sigma6 = t6 * t949 + f64x8::splat(2.0) * t426;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t952 = t5 * t356;
            let t964 = t29 * t33 * t368 * t72;
            let t968 = t230 * t380 * t441;
            let t971 = t825 * t63;
            let t973 = t548 / t971;
            let t975 = t376 * t554 * t72;
            let t976 = t973 * t975;
            let t980 = ((t53).select(f64x8::splat(0.0), -t952 * t173 / f64x8::splat(64.0) - t929 / f64x8::splat(96.0) + t428 * t433 / f64x8::splat(12.0) - t837 * t442 / f64x8::splat(288.0) + t905 + t934 / f64x8::splat(36.0) - t938 / f64x8::splat(864.0) - f64x8::splat(11.0) / f64x8::splat(72.0) * t170 * t964 + t375 * t968 / f64x8::splat(64.0) - t844 * t976 / f64x8::splat(864.0)));
            let tv3rho2sigma8 = t6 * t980 + f64x8::splat(2.0) * t446;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t985 = t504 * t453 / f64x8::splat(4608.0);
            let t988 = t230 * t403 * t41 * t45;
            let t991 = t487 * v_rho0;
            let t993 = t548 / t991;
            let t995 = t554 * v_sigma0 * t45;
            let t996 = t993 * t995;
            let t1000 = ((t1).select(f64x8::splat(0.0), t498 * t453 / f64x8::splat(1536.0) + t985 - t226 * t988 / f64x8::splat(288.0) + t547 * t996 / f64x8::splat(2304.0)));
            let tv3rhosigma20 = t6 * t1000 + t456;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1005 = t768 * t462 / f64x8::splat(4608.0);
            let t1007 = ((t53).select(f64x8::splat(0.0), t764 * t462 / f64x8::splat(1536.0) + t1005));
            let tv3rhosigma25 = t6 * t1007 + t465;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1012 = ((t1).select(f64x8::splat(0.0), t639 * t453 / f64x8::splat(1536.0) + t985));
            let tv3rhosigma26 = t6 * t1012 + t456;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1018 = t230 * t438 * t41 * t72;
            let t1021 = t825 * v_rho1;
            let t1023 = t548 / t1021;
            let t1025 = t554 * v_sigma2 * t72;
            let t1026 = t1023 * t1025;
            let t1030 = ((t53).select(f64x8::splat(0.0), t837 * t462 / f64x8::splat(1536.0) + t1005 - t375 * t1018 / f64x8::splat(288.0) + t844 * t1026 / f64x8::splat(2304.0)));
            let tv3rhosigma211 = t6 * t1030 + t465;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1034 = t554 * t45;
            let t1035 = t548 / t487 * t1034;
            let t1038 = ((t1).select(f64x8::splat(0.0), -t547 * t1035 / f64x8::splat(6144.0)));
            let tv3sigma30 = t6 * t1038;
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
            let t1041 = t554 * t72;
            let t1042 = t548 / t825 * t1041;
            let t1045 = ((t53).select(f64x8::splat(0.0), -t844 * t1042 / f64x8::splat(6144.0)));
            let tv3sigma39 = t6 * t1045;
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
