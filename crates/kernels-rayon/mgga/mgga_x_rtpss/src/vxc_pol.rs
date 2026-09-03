//! MGGA_X_RTPSS vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rtpss.c`
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
pub fn mgga_x_rtpss_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_e = f64x8::splat(param_e);
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
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
            let t29 = v_sigma0 * v_sigma0;
            let t30 = param_c * t29;
            let t31 = v_rho0 * v_rho0;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = v_tau0 * v_tau0;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t32 * t34;
            let t36 = t29 * t32;
            let t37 = t36 * t34;
            let t39 = f64x8::splat(1.0) + t37 / f64x8::splat(64.0);
            let t40 = t39 * t39;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t35 * t41;
            let t46 = f64x8::splat(M_CBRT6);
            let t47 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t30 * t42 / f64x8::splat(64.0)) * t46;
            let t48 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t49 = (simd::cbrt(t48));
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t51 * v_sigma0;
            let t53 = (simd::cbrt(v_rho0));
            let t54 = t53 * t53;
            let t56 = f64x8::splat(1.0) / t54 / t31;
            let t57 = t52 * t56;
            let t61 = f64x8::splat(1.0) / t54 / v_rho0;
            let t63 = v_sigma0 * t56;
            let t65 = v_tau0 * t61 - t63 / f64x8::splat(8.0);
            let t69 = f64x8::splat(5.0) / f64x8::splat(9.0) * t65 * t46 * t51 - f64x8::splat(1.0);
            let t70 = param_b * t65;
            let t71 = t46 * t51;
            let t72 = t71 * t69;
            let t75 = f64x8::splat(5.0) * t70 * t72 + f64x8::splat(9.0);
            let t76 = ((t75).sqrt());
            let t77 = f64x8::splat(1.0) / t76;
            let t82 = f64x8::splat(27.0) / f64x8::splat(20.0) * t69 * t77 + t71 * t63 / f64x8::splat(36.0);
            let t83 = t82 * t82;
            let t86 = t46 * t46;
            let t88 = f64x8::splat(1.0) / t49 / t48;
            let t89 = t86 * t88;
            let t90 = t31 * t31;
            let t91 = t90 * v_rho0;
            let t93 = f64x8::splat(1.0) / t53 / t91;
            let t97 = f64x8::splat(50.0) * t89 * t29 * t93 + f64x8::splat(162.0) * t37;
            let t98 = ((t97).sqrt());
            let t101 = f64x8::splat(1.0) / param_kappa;
            let t102 = t101 * t86;
            let t103 = t88 * t29;
            let t107 = ((param_e).sqrt());
            let t108 = t107 * t29;
            let t111 = param_e * param_mu;
            let t112 = t48 * t48;
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t29 * v_sigma0;
            let t115 = t113 * t114;
            let t116 = t90 * t90;
            let t117 = f64x8::splat(1.0) / t116;
            let t121 = t47 * t57 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t83 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t82 * t98 + f64x8::splat(25.0) / f64x8::splat(944784.0) * t102 * t103 * t93 + t108 * t35 / f64x8::splat(720.0) + t111 * t115 * t117 / f64x8::splat(2304.0);
            let t122 = t107 * t46;
            let t125 = f64x8::splat(1.0) + t122 * t57 / f64x8::splat(24.0);
            let t126 = t125 * t125;
            let t127 = f64x8::splat(1.0) / t126;
            let t130 = (simd::exp(-t121 * t127 * t101));
            let t133 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t130);
            let t137 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t133));
            let t138 = (v_rho1).simd_le(dens_threshold);
            let t139 = -t17;
            let t141 = ((t15).select(t12, (t11).select(t16, t139 * t8)));
            let t142 = f64x8::splat(1.0) + t141;
            let t143 = (t142).simd_le(zeta_threshold);
            let t144 = (simd::cbrt(t142));
            let t146 = ((t143).select(t23, t144 * t142));
            let t147 = t146 * t27;
            let t148 = v_sigma2 * v_sigma2;
            let t149 = param_c * t148;
            let t150 = v_rho1 * v_rho1;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = v_tau1 * v_tau1;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t151 * t153;
            let t155 = t148 * t151;
            let t156 = t155 * t153;
            let t158 = f64x8::splat(1.0) + t156 / f64x8::splat(64.0);
            let t159 = t158 * t158;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t154 * t160;
            let t165 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t149 * t161 / f64x8::splat(64.0)) * t46;
            let t166 = t51 * v_sigma2;
            let t167 = (simd::cbrt(v_rho1));
            let t168 = t167 * t167;
            let t170 = f64x8::splat(1.0) / t168 / t150;
            let t171 = t166 * t170;
            let t175 = f64x8::splat(1.0) / t168 / v_rho1;
            let t177 = v_sigma2 * t170;
            let t179 = v_tau1 * t175 - t177 / f64x8::splat(8.0);
            let t183 = f64x8::splat(5.0) / f64x8::splat(9.0) * t179 * t46 * t51 - f64x8::splat(1.0);
            let t184 = param_b * t179;
            let t185 = t71 * t183;
            let t188 = f64x8::splat(5.0) * t184 * t185 + f64x8::splat(9.0);
            let t189 = ((t188).sqrt());
            let t190 = f64x8::splat(1.0) / t189;
            let t195 = f64x8::splat(27.0) / f64x8::splat(20.0) * t183 * t190 + t71 * t177 / f64x8::splat(36.0);
            let t196 = t195 * t195;
            let t199 = t150 * t150;
            let t200 = t199 * v_rho1;
            let t202 = f64x8::splat(1.0) / t167 / t200;
            let t206 = f64x8::splat(50.0) * t89 * t148 * t202 + f64x8::splat(162.0) * t156;
            let t207 = ((t206).sqrt());
            let t210 = t88 * t148;
            let t214 = t107 * t148;
            let t217 = t148 * v_sigma2;
            let t218 = t113 * t217;
            let t219 = t199 * t199;
            let t220 = f64x8::splat(1.0) / t219;
            let t224 = t165 * t171 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t196 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t195 * t207 + f64x8::splat(25.0) / f64x8::splat(944784.0) * t102 * t210 * t202 + t214 * t154 / f64x8::splat(720.0) + t111 * t218 * t220 / f64x8::splat(2304.0);
            let t227 = f64x8::splat(1.0) + t122 * t171 / f64x8::splat(24.0);
            let t228 = t227 * t227;
            let t229 = f64x8::splat(1.0) / t228;
            let t232 = (simd::exp(-t224 * t229 * t101));
            let t235 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t232);
            let t239 = ((t138).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t147 * t235));
            let tzk0 = t137 + t239;
            acc_zk = tzk0;
            let t240 = t7 * t7;
            let t241 = f64x8::splat(1.0) / t240;
            let t242 = t17 * t241;
            let t244 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t242)));
            let t247 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t244));
            let t248 = t247 * t27;
            let t252 = t27 * t27;
            let t253 = f64x8::splat(1.0) / t252;
            let t254 = t26 * t253;
            let t257 = t6 * t254 * t133 / f64x8::splat(8.0);
            let t258 = t6 * t26;
            let t259 = t27 * param_kappa;
            let t260 = t31 * v_rho0;
            let t261 = f64x8::splat(1.0) / t260;
            let t262 = t261 * t34;
            let t263 = t262 * t41;
            let t266 = t29 * t29;
            let t267 = param_c * t266;
            let t268 = f64x8::splat(1.0) / t91;
            let t269 = t33 * t33;
            let t270 = f64x8::splat(1.0) / t269;
            let t273 = f64x8::splat(1.0) / t40 / t39;
            let t274 = t268 * t270 * t273;
            let t278 = (-t30 * t263 / f64x8::splat(32.0) + t267 * t274 / f64x8::splat(1024.0)) * t46;
            let t282 = f64x8::splat(1.0) / t54 / t260;
            let t283 = t52 * t282;
            let t288 = v_sigma0 * t282;
            let t290 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t56 + t288 / f64x8::splat(3.0);
            let t291 = t290 * t46;
            let t292 = t51 * t77;
            let t296 = f64x8::splat(1.0) / t76 / t75;
            let t297 = t69 * t296;
            let t301 = t89 * t290;
            let t304 = f64x8::splat(5.0) * param_b * t290 * t72 + f64x8::splat(25.0) / f64x8::splat(9.0) * t70 * t301;
            let t307 = t71 * t288;
            let t309 = f64x8::splat(3.0) / f64x8::splat(4.0) * t291 * t292 - f64x8::splat(27.0) / f64x8::splat(40.0) * t297 * t304 - f64x8::splat(2.0) / f64x8::splat(27.0) * t307;
            let t314 = f64x8::splat(1.0) / t98;
            let t315 = t82 * t314;
            let t316 = t29 * t261;
            let t319 = t90 * t31;
            let t321 = f64x8::splat(1.0) / t53 / t319;
            let t325 = -f64x8::splat(324.0) * t316 * t34 - f64x8::splat(800.0) / f64x8::splat(3.0) * t89 * t29 * t321;
            let t333 = t116 * v_rho0;
            let t334 = f64x8::splat(1.0) / t333;
            let t338 = t278 * t57 / f64x8::splat(24.0) - t47 * t283 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t82 * t309 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t309 * t98 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t315 * t325 - f64x8::splat(25.0) / f64x8::splat(177147.0) * t102 * t103 * t321 - t108 * t262 / f64x8::splat(360.0) - t111 * t115 * t334 / f64x8::splat(288.0);
            let t341 = t126 * t125;
            let t342 = f64x8::splat(1.0) / t341;
            let t343 = t121 * t342;
            let t344 = t101 * t107;
            let t345 = t343 * t344;
            let t348 = -t338 * t127 * t101 - f64x8::splat(2.0) / f64x8::splat(9.0) * t345 * t307;
            let t349 = t348 * t130;
            let t350 = t259 * t349;
            let t354 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t248 * t133 - t257 + f64x8::splat(3.0) / f64x8::splat(8.0) * t258 * t350));
            let t355 = t139 * t241;
            let t357 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t355)));
            let t360 = ((t143).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t144 * t357));
            let t361 = t360 * t27;
            let t365 = t146 * t253;
            let t368 = t6 * t365 * t235 / f64x8::splat(8.0);
            let t370 = ((t138).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t361 * t235 - t368));
            let tvrho0 = t137 + t239 + t7 * (t354 + t370);
            acc_vrho_0 = tvrho0;
            let t374 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t242)));
            let t377 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t374));
            let t378 = t377 * t27;
            let t383 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t378 * t133 - t257));
            let t385 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t355)));
            let t388 = ((t143).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t144 * t385));
            let t389 = t388 * t27;
            let t393 = t6 * t146;
            let t394 = t150 * v_rho1;
            let t395 = f64x8::splat(1.0) / t394;
            let t396 = t395 * t153;
            let t397 = t396 * t160;
            let t400 = t148 * t148;
            let t401 = param_c * t400;
            let t402 = f64x8::splat(1.0) / t200;
            let t403 = t152 * t152;
            let t404 = f64x8::splat(1.0) / t403;
            let t407 = f64x8::splat(1.0) / t159 / t158;
            let t408 = t402 * t404 * t407;
            let t412 = (-t149 * t397 / f64x8::splat(32.0) + t401 * t408 / f64x8::splat(1024.0)) * t46;
            let t416 = f64x8::splat(1.0) / t168 / t394;
            let t417 = t166 * t416;
            let t422 = v_sigma2 * t416;
            let t424 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t170 + t422 / f64x8::splat(3.0);
            let t425 = t424 * t46;
            let t426 = t51 * t190;
            let t430 = f64x8::splat(1.0) / t189 / t188;
            let t431 = t183 * t430;
            let t435 = t89 * t424;
            let t438 = f64x8::splat(5.0) * param_b * t424 * t185 + f64x8::splat(25.0) / f64x8::splat(9.0) * t184 * t435;
            let t441 = t71 * t422;
            let t443 = f64x8::splat(3.0) / f64x8::splat(4.0) * t425 * t426 - f64x8::splat(27.0) / f64x8::splat(40.0) * t431 * t438 - f64x8::splat(2.0) / f64x8::splat(27.0) * t441;
            let t448 = f64x8::splat(1.0) / t207;
            let t449 = t195 * t448;
            let t450 = t148 * t395;
            let t453 = t199 * t150;
            let t455 = f64x8::splat(1.0) / t167 / t453;
            let t459 = -f64x8::splat(324.0) * t450 * t153 - f64x8::splat(800.0) / f64x8::splat(3.0) * t89 * t148 * t455;
            let t467 = t219 * v_rho1;
            let t468 = f64x8::splat(1.0) / t467;
            let t472 = t412 * t171 / f64x8::splat(24.0) - t165 * t417 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t195 * t443 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t443 * t207 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t449 * t459 - f64x8::splat(25.0) / f64x8::splat(177147.0) * t102 * t210 * t455 - t214 * t396 / f64x8::splat(360.0) - t111 * t218 * t468 / f64x8::splat(288.0);
            let t475 = t228 * t227;
            let t476 = f64x8::splat(1.0) / t475;
            let t477 = t224 * t476;
            let t478 = t477 * t344;
            let t481 = -t472 * t229 * t101 - f64x8::splat(2.0) / f64x8::splat(9.0) * t478 * t441;
            let t482 = t481 * t232;
            let t483 = t259 * t482;
            let t487 = ((t138).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t389 * t235 - t368 + f64x8::splat(3.0) / f64x8::splat(8.0) * t393 * t483));
            let tvrho1 = t137 + t239 + t7 * (t383 + t487);
            acc_vrho_1 = tvrho1;
            let t490 = param_c * v_sigma0;
            let t493 = param_c * t114;
            let t494 = f64x8::splat(1.0) / t90;
            let t495 = t494 * t270;
            let t496 = t495 * t273;
            let t500 = (t490 * t42 / f64x8::splat(32.0) - t493 * t496 / f64x8::splat(1024.0)) * t46;
            let t503 = t51 * t56;
            let t506 = t56 * t46;
            let t507 = t506 * t292;
            let t509 = param_b * t56;
            let t510 = t509 * t72;
            let t513 = t70 * t89 * t56;
            let t515 = -f64x8::splat(5.0) / f64x8::splat(8.0) * t510 - f64x8::splat(25.0) / f64x8::splat(72.0) * t513;
            let t518 = t506 * t51;
            let t520 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t507 - f64x8::splat(27.0) / f64x8::splat(40.0) * t297 * t515 + t518 / f64x8::splat(36.0);
            let t525 = v_sigma0 * t32;
            let t531 = f64x8::splat(100.0) * t89 * v_sigma0 * t93 + f64x8::splat(324.0) * t525 * t34;
            let t534 = t88 * v_sigma0;
            let t538 = t107 * v_sigma0;
            let t541 = t113 * t29;
            let t545 = t500 * t57 / f64x8::splat(24.0) + t47 * t503 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t82 * t520 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t520 * t98 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t315 * t531 + f64x8::splat(25.0) / f64x8::splat(472392.0) * t102 * t534 * t93 + t538 * t35 / f64x8::splat(360.0) + t111 * t541 * t117 / f64x8::splat(768.0);
            let t548 = t343 * t101;
            let t549 = t122 * t503;
            let t552 = -t545 * t127 * t101 + t548 * t549 / f64x8::splat(12.0);
            let t553 = t552 * t130;
            let t554 = t259 * t553;
            let t557 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(8.0) * t258 * t554));
            let tvsigma0 = t7 * t557;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t558 = param_c * v_sigma2;
            let t561 = param_c * t217;
            let t562 = f64x8::splat(1.0) / t199;
            let t563 = t562 * t404;
            let t564 = t563 * t407;
            let t568 = (t558 * t161 / f64x8::splat(32.0) - t561 * t564 / f64x8::splat(1024.0)) * t46;
            let t571 = t51 * t170;
            let t574 = t170 * t46;
            let t575 = t574 * t426;
            let t577 = param_b * t170;
            let t578 = t577 * t185;
            let t581 = t184 * t89 * t170;
            let t583 = -f64x8::splat(5.0) / f64x8::splat(8.0) * t578 - f64x8::splat(25.0) / f64x8::splat(72.0) * t581;
            let t586 = t574 * t51;
            let t588 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t575 - f64x8::splat(27.0) / f64x8::splat(40.0) * t431 * t583 + t586 / f64x8::splat(36.0);
            let t593 = v_sigma2 * t151;
            let t599 = f64x8::splat(100.0) * t89 * v_sigma2 * t202 + f64x8::splat(324.0) * t593 * t153;
            let t602 = t88 * v_sigma2;
            let t606 = t107 * v_sigma2;
            let t609 = t113 * t148;
            let t613 = t568 * t171 / f64x8::splat(24.0) + t165 * t571 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t195 * t588 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t588 * t207 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t449 * t599 + f64x8::splat(25.0) / f64x8::splat(472392.0) * t102 * t602 * t202 + t606 * t154 / f64x8::splat(360.0) + t111 * t609 * t220 / f64x8::splat(768.0);
            let t616 = t477 * t101;
            let t617 = t122 * t571;
            let t620 = -t613 * t229 * t101 + t616 * t617 / f64x8::splat(12.0);
            let t621 = t620 * t232;
            let t622 = t259 * t621;
            let t625 = ((t138).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(8.0) * t393 * t622));
            let tvsigma2 = t7 * t625;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t626 = t33 * v_tau0;
            let t627 = f64x8::splat(1.0) / t626;
            let t628 = t32 * t627;
            let t629 = t628 * t41;
            let t632 = t269 * v_tau0;
            let t633 = f64x8::splat(1.0) / t632;
            let t635 = t494 * t633 * t273;
            let t639 = (-t30 * t629 / f64x8::splat(32.0) + t267 * t635 / f64x8::splat(1024.0)) * t46;
            let t642 = t61 * t46;
            let t645 = param_b * t61;
            let t651 = f64x8::splat(5.0) * t645 * t72 + f64x8::splat(25.0) / f64x8::splat(9.0) * t70 * t89 * t61;
            let t654 = f64x8::splat(3.0) / f64x8::splat(4.0) * t642 * t292 - f64x8::splat(27.0) / f64x8::splat(40.0) * t297 * t651;
            let t659 = t36 * t627;
            let t664 = t639 * t57 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t82 * t654 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t654 * t98 + f64x8::splat(73.0) / f64x8::splat(600.0) * t315 * t659 - t108 * t628 / f64x8::splat(360.0);
            let t665 = t27 * t664;
            let t666 = t127 * t130;
            let t667 = t665 * t666;
            let t670 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t258 * t667));
            let tvtau0 = t7 * t670;
            acc_vtau_0 = tvtau0;
            let t671 = t152 * v_tau1;
            let t672 = f64x8::splat(1.0) / t671;
            let t673 = t151 * t672;
            let t674 = t673 * t160;
            let t677 = t403 * v_tau1;
            let t678 = f64x8::splat(1.0) / t677;
            let t680 = t562 * t678 * t407;
            let t684 = (-t149 * t674 / f64x8::splat(32.0) + t401 * t680 / f64x8::splat(1024.0)) * t46;
            let t687 = t175 * t46;
            let t690 = param_b * t175;
            let t696 = f64x8::splat(5.0) * t690 * t185 + f64x8::splat(25.0) / f64x8::splat(9.0) * t184 * t89 * t175;
            let t699 = f64x8::splat(3.0) / f64x8::splat(4.0) * t687 * t426 - f64x8::splat(27.0) / f64x8::splat(40.0) * t431 * t696;
            let t704 = t155 * t672;
            let t709 = t684 * t171 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t195 * t699 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t699 * t207 + f64x8::splat(73.0) / f64x8::splat(600.0) * t449 * t704 - t214 * t673 / f64x8::splat(360.0);
            let t710 = t27 * t709;
            let t711 = t229 * t232;
            let t712 = t710 * t711;
            let t715 = ((t138).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t393 * t712));
            let tvtau1 = t7 * t715;
            acc_vtau_1 = tvtau1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}
