//! MGGA_X_TPSS vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tpss.c`
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
pub fn mgga_x_tpss_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_BLOC_a: f64,
    param_BLOC_b: f64,
    param_b: f64,
    param_c: f64,
    param_e: f64,
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_BLOC_a = f64x8::splat(param_BLOC_a);
    let param_BLOC_b = f64x8::splat(param_BLOC_b);
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
            let t29 = f64x8::splat(1.0) / v_rho0;
            let t31 = f64x8::splat(1.0) / v_tau0;
            let t33 = v_sigma0 * t29 * t31 / f64x8::splat(8.0);
            let t34 = param_BLOC_b * v_sigma0;
            let t38 = param_BLOC_a + t34 * t29 * t31 / f64x8::splat(8.0);
            let t39 = (simd::pow(t33, t38));
            let t40 = param_c * t39;
            let t41 = v_sigma0 * v_sigma0;
            let t42 = v_rho0 * v_rho0;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t41 * t43;
            let t45 = v_tau0 * v_tau0;
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = t44 * t46;
            let t49 = f64x8::splat(1.0) + t47 / f64x8::splat(64.0);
            let t50 = t49 * t49;
            let t51 = f64x8::splat(1.0) / t50;
            let t54 = f64x8::splat(M_CBRT6);
            let t55 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t40 * t51) * t54;
            let t56 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t57 = (simd::cbrt(t56));
            let t58 = t57 * t57;
            let t59 = f64x8::splat(1.0) / t58;
            let t60 = t59 * v_sigma0;
            let t61 = (simd::cbrt(v_rho0));
            let t62 = t61 * t61;
            let t64 = f64x8::splat(1.0) / t62 / t42;
            let t65 = t60 * t64;
            let t69 = f64x8::splat(1.0) / t62 / v_rho0;
            let t71 = v_sigma0 * t64;
            let t73 = v_tau0 * t69 - t71 / f64x8::splat(8.0);
            let t77 = f64x8::splat(5.0) / f64x8::splat(9.0) * t73 * t54 * t59 - f64x8::splat(1.0);
            let t78 = param_b * t73;
            let t79 = t54 * t59;
            let t80 = t79 * t77;
            let t83 = f64x8::splat(5.0) * t78 * t80 + f64x8::splat(9.0);
            let t84 = ((t83).sqrt());
            let t85 = f64x8::splat(1.0) / t84;
            let t90 = f64x8::splat(27.0) / f64x8::splat(20.0) * t77 * t85 + t79 * t71 / f64x8::splat(36.0);
            let t91 = t90 * t90;
            let t94 = t54 * t54;
            let t96 = f64x8::splat(1.0) / t57 / t56;
            let t97 = t94 * t96;
            let t98 = t42 * t42;
            let t99 = t98 * v_rho0;
            let t101 = f64x8::splat(1.0) / t61 / t99;
            let t105 = f64x8::splat(50.0) * t97 * t41 * t101 + f64x8::splat(162.0) * t47;
            let t106 = ((t105).sqrt());
            let t110 = f64x8::splat(1.0) / param_kappa * t94;
            let t111 = t96 * t41;
            let t115 = ((param_e).sqrt());
            let t116 = t115 * t41;
            let t117 = t43 * t46;
            let t120 = param_e * param_mu;
            let t121 = t56 * t56;
            let t122 = f64x8::splat(1.0) / t121;
            let t123 = t41 * v_sigma0;
            let t124 = t122 * t123;
            let t125 = t98 * t98;
            let t126 = f64x8::splat(1.0) / t125;
            let t130 = t55 * t65 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t91 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t90 * t106 + f64x8::splat(25.0) / f64x8::splat(944784.0) * t110 * t111 * t101 + t116 * t117 / f64x8::splat(720.0) + t120 * t124 * t126 / f64x8::splat(2304.0);
            let t131 = t115 * t54;
            let t134 = f64x8::splat(1.0) + t131 * t65 / f64x8::splat(24.0);
            let t135 = t134 * t134;
            let t136 = f64x8::splat(1.0) / t135;
            let t138 = t130 * t136 + param_kappa;
            let t143 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t138);
            let t147 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t143));
            let t148 = (v_rho1).simd_le(dens_threshold);
            let t149 = -t17;
            let t151 = ((t15).select(t12, (t11).select(t16, t149 * t8)));
            let t152 = f64x8::splat(1.0) + t151;
            let t153 = (t152).simd_le(zeta_threshold);
            let t154 = (simd::cbrt(t152));
            let t156 = ((t153).select(t23, t154 * t152));
            let t157 = t156 * t27;
            let t158 = f64x8::splat(1.0) / v_rho1;
            let t160 = f64x8::splat(1.0) / v_tau1;
            let t162 = v_sigma2 * t158 * t160 / f64x8::splat(8.0);
            let t163 = param_BLOC_b * v_sigma2;
            let t167 = param_BLOC_a + t163 * t158 * t160 / f64x8::splat(8.0);
            let t168 = (simd::pow(t162, t167));
            let t169 = param_c * t168;
            let t170 = v_sigma2 * v_sigma2;
            let t171 = v_rho1 * v_rho1;
            let t172 = f64x8::splat(1.0) / t171;
            let t173 = t170 * t172;
            let t174 = v_tau1 * v_tau1;
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t173 * t175;
            let t178 = f64x8::splat(1.0) + t176 / f64x8::splat(64.0);
            let t179 = t178 * t178;
            let t180 = f64x8::splat(1.0) / t179;
            let t183 = (f64x8::splat(10.0) / f64x8::splat(81.0) + t169 * t180) * t54;
            let t184 = t59 * v_sigma2;
            let t185 = (simd::cbrt(v_rho1));
            let t186 = t185 * t185;
            let t188 = f64x8::splat(1.0) / t186 / t171;
            let t189 = t184 * t188;
            let t193 = f64x8::splat(1.0) / t186 / v_rho1;
            let t195 = v_sigma2 * t188;
            let t197 = v_tau1 * t193 - t195 / f64x8::splat(8.0);
            let t201 = f64x8::splat(5.0) / f64x8::splat(9.0) * t197 * t54 * t59 - f64x8::splat(1.0);
            let t202 = param_b * t197;
            let t203 = t79 * t201;
            let t206 = f64x8::splat(5.0) * t202 * t203 + f64x8::splat(9.0);
            let t207 = ((t206).sqrt());
            let t208 = f64x8::splat(1.0) / t207;
            let t213 = f64x8::splat(27.0) / f64x8::splat(20.0) * t201 * t208 + t79 * t195 / f64x8::splat(36.0);
            let t214 = t213 * t213;
            let t217 = t171 * t171;
            let t218 = t217 * v_rho1;
            let t220 = f64x8::splat(1.0) / t185 / t218;
            let t224 = f64x8::splat(50.0) * t97 * t170 * t220 + f64x8::splat(162.0) * t176;
            let t225 = ((t224).sqrt());
            let t228 = t96 * t170;
            let t232 = t115 * t170;
            let t233 = t172 * t175;
            let t236 = t170 * v_sigma2;
            let t237 = t122 * t236;
            let t238 = t217 * t217;
            let t239 = f64x8::splat(1.0) / t238;
            let t243 = t183 * t189 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t214 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t213 * t225 + f64x8::splat(25.0) / f64x8::splat(944784.0) * t110 * t228 * t220 + t232 * t233 / f64x8::splat(720.0) + t120 * t237 * t239 / f64x8::splat(2304.0);
            let t246 = f64x8::splat(1.0) + t131 * t189 / f64x8::splat(24.0);
            let t247 = t246 * t246;
            let t248 = f64x8::splat(1.0) / t247;
            let t250 = t243 * t248 + param_kappa;
            let t255 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t250);
            let t259 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t157 * t255));
            let tzk0 = t147 + t259;
            acc_zk = tzk0;
            let t260 = t7 * t7;
            let t261 = f64x8::splat(1.0) / t260;
            let t262 = t17 * t261;
            let t264 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t262)));
            let t267 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t264));
            let t268 = t267 * t27;
            let t272 = t27 * t27;
            let t273 = f64x8::splat(1.0) / t272;
            let t274 = t26 * t273;
            let t277 = t6 * t274 * t143 / f64x8::splat(8.0);
            let t278 = t6 * t26;
            let t279 = param_kappa * param_kappa;
            let t280 = t27 * t279;
            let t281 = t138 * t138;
            let t282 = f64x8::splat(1.0) / t281;
            let t283 = t43 * t31;
            let t284 = (simd::ln(t33));
            let t289 = -t34 * t283 * t284 / f64x8::splat(8.0) - t38 * t29;
            let t290 = t289 * t51;
            let t293 = f64x8::splat(1.0) / t50 / t49;
            let t294 = t40 * t293;
            let t295 = t42 * v_rho0;
            let t296 = f64x8::splat(1.0) / t295;
            let t297 = t41 * t296;
            let t298 = t297 * t46;
            let t302 = (t40 * t290 + t294 * t298 / f64x8::splat(16.0)) * t54;
            let t306 = f64x8::splat(1.0) / t62 / t295;
            let t307 = t60 * t306;
            let t312 = v_sigma0 * t306;
            let t314 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t64 + t312 / f64x8::splat(3.0);
            let t315 = t314 * t54;
            let t316 = t59 * t85;
            let t320 = f64x8::splat(1.0) / t84 / t83;
            let t321 = t77 * t320;
            let t325 = t97 * t314;
            let t328 = f64x8::splat(5.0) * param_b * t314 * t80 + f64x8::splat(25.0) / f64x8::splat(9.0) * t78 * t325;
            let t331 = t79 * t312;
            let t333 = f64x8::splat(3.0) / f64x8::splat(4.0) * t315 * t316 - f64x8::splat(27.0) / f64x8::splat(40.0) * t321 * t328 - f64x8::splat(2.0) / f64x8::splat(27.0) * t331;
            let t338 = f64x8::splat(1.0) / t106;
            let t339 = t90 * t338;
            let t341 = t98 * t42;
            let t343 = f64x8::splat(1.0) / t61 / t341;
            let t347 = -f64x8::splat(324.0) * t298 - f64x8::splat(800.0) / f64x8::splat(3.0) * t97 * t41 * t343;
            let t353 = t296 * t46;
            let t356 = t125 * v_rho0;
            let t357 = f64x8::splat(1.0) / t356;
            let t361 = t302 * t65 / f64x8::splat(24.0) - t55 * t307 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t90 * t333 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t333 * t106 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t339 * t347 - f64x8::splat(25.0) / f64x8::splat(177147.0) * t110 * t111 * t343 - t116 * t353 / f64x8::splat(360.0) - t120 * t124 * t357 / f64x8::splat(288.0);
            let t363 = t135 * t134;
            let t364 = f64x8::splat(1.0) / t363;
            let t366 = t130 * t364 * t115;
            let t369 = t361 * t136 + f64x8::splat(2.0) / f64x8::splat(9.0) * t366 * t331;
            let t370 = t282 * t369;
            let t371 = t280 * t370;
            let t375 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t268 * t143 - t277 - f64x8::splat(3.0) / f64x8::splat(8.0) * t278 * t371));
            let t376 = t149 * t261;
            let t378 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t376)));
            let t381 = ((t153).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t154 * t378));
            let t382 = t381 * t27;
            let t386 = t156 * t273;
            let t389 = t6 * t386 * t255 / f64x8::splat(8.0);
            let t391 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t382 * t255 - t389));
            let tvrho0 = t147 + t259 + t7 * (t375 + t391);
            acc_vrho_0 = tvrho0;
            let t395 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t262)));
            let t398 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t395));
            let t399 = t398 * t27;
            let t404 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t399 * t143 - t277));
            let t406 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t376)));
            let t409 = ((t153).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t154 * t406));
            let t410 = t409 * t27;
            let t414 = t6 * t156;
            let t415 = t250 * t250;
            let t416 = f64x8::splat(1.0) / t415;
            let t417 = t172 * t160;
            let t418 = (simd::ln(t162));
            let t423 = -t163 * t417 * t418 / f64x8::splat(8.0) - t167 * t158;
            let t424 = t423 * t180;
            let t427 = f64x8::splat(1.0) / t179 / t178;
            let t428 = t169 * t427;
            let t429 = t171 * v_rho1;
            let t430 = f64x8::splat(1.0) / t429;
            let t431 = t170 * t430;
            let t432 = t431 * t175;
            let t436 = (t169 * t424 + t428 * t432 / f64x8::splat(16.0)) * t54;
            let t440 = f64x8::splat(1.0) / t186 / t429;
            let t441 = t184 * t440;
            let t446 = v_sigma2 * t440;
            let t448 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t188 + t446 / f64x8::splat(3.0);
            let t449 = t448 * t54;
            let t450 = t59 * t208;
            let t454 = f64x8::splat(1.0) / t207 / t206;
            let t455 = t201 * t454;
            let t459 = t97 * t448;
            let t462 = f64x8::splat(5.0) * param_b * t448 * t203 + f64x8::splat(25.0) / f64x8::splat(9.0) * t202 * t459;
            let t465 = t79 * t446;
            let t467 = f64x8::splat(3.0) / f64x8::splat(4.0) * t449 * t450 - f64x8::splat(27.0) / f64x8::splat(40.0) * t455 * t462 - f64x8::splat(2.0) / f64x8::splat(27.0) * t465;
            let t472 = f64x8::splat(1.0) / t225;
            let t473 = t213 * t472;
            let t475 = t217 * t171;
            let t477 = f64x8::splat(1.0) / t185 / t475;
            let t481 = -f64x8::splat(324.0) * t432 - f64x8::splat(800.0) / f64x8::splat(3.0) * t97 * t170 * t477;
            let t487 = t430 * t175;
            let t490 = t238 * v_rho1;
            let t491 = f64x8::splat(1.0) / t490;
            let t495 = t436 * t189 / f64x8::splat(24.0) - t183 * t441 / f64x8::splat(9.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t213 * t467 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t467 * t225 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t473 * t481 - f64x8::splat(25.0) / f64x8::splat(177147.0) * t110 * t228 * t477 - t232 * t487 / f64x8::splat(360.0) - t120 * t237 * t491 / f64x8::splat(288.0);
            let t497 = t247 * t246;
            let t498 = f64x8::splat(1.0) / t497;
            let t500 = t243 * t498 * t115;
            let t503 = t495 * t248 + f64x8::splat(2.0) / f64x8::splat(9.0) * t500 * t465;
            let t504 = t416 * t503;
            let t505 = t280 * t504;
            let t509 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t410 * t255 - t389 - f64x8::splat(3.0) / f64x8::splat(8.0) * t414 * t505));
            let tvrho1 = t147 + t259 + t7 * (t404 + t509);
            acc_vrho_1 = tvrho1;
            let t512 = param_BLOC_b * t29;
            let t513 = t31 * t284;
            let t516 = f64x8::splat(1.0) / v_sigma0;
            let t518 = t512 * t513 / f64x8::splat(8.0) + t38 * t516;
            let t519 = t518 * t51;
            let t520 = t40 * t519;
            let t521 = v_sigma0 * t43;
            let t522 = t521 * t46;
            let t526 = (t520 - t294 * t522 / f64x8::splat(16.0)) * t54;
            let t529 = t59 * t64;
            let t532 = t64 * t54;
            let t533 = t532 * t316;
            let t535 = param_b * t64;
            let t536 = t535 * t80;
            let t539 = t78 * t97 * t64;
            let t541 = -f64x8::splat(5.0) / f64x8::splat(8.0) * t536 - f64x8::splat(25.0) / f64x8::splat(72.0) * t539;
            let t544 = t532 * t59;
            let t546 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t533 - f64x8::splat(27.0) / f64x8::splat(40.0) * t321 * t541 + t544 / f64x8::splat(36.0);
            let t555 = f64x8::splat(100.0) * t97 * v_sigma0 * t101 + f64x8::splat(324.0) * t522;
            let t558 = t96 * v_sigma0;
            let t562 = t115 * v_sigma0;
            let t565 = t122 * t41;
            let t569 = t526 * t65 / f64x8::splat(24.0) + t55 * t529 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t90 * t546 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t546 * t106 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t339 * t555 + f64x8::splat(25.0) / f64x8::splat(472392.0) * t110 * t558 * t101 + t562 * t117 / f64x8::splat(360.0) + t120 * t565 * t126 / f64x8::splat(768.0);
            let t573 = t569 * t136 - t366 * t544 / f64x8::splat(12.0);
            let t574 = t282 * t573;
            let t575 = t280 * t574;
            let t578 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t278 * t575));
            let tvsigma0 = t7 * t578;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t579 = param_BLOC_b * t158;
            let t580 = t160 * t418;
            let t583 = f64x8::splat(1.0) / v_sigma2;
            let t585 = t579 * t580 / f64x8::splat(8.0) + t167 * t583;
            let t586 = t585 * t180;
            let t587 = t169 * t586;
            let t588 = v_sigma2 * t172;
            let t589 = t588 * t175;
            let t593 = (t587 - t428 * t589 / f64x8::splat(16.0)) * t54;
            let t596 = t59 * t188;
            let t599 = t188 * t54;
            let t600 = t599 * t450;
            let t602 = param_b * t188;
            let t603 = t602 * t203;
            let t606 = t202 * t97 * t188;
            let t608 = -f64x8::splat(5.0) / f64x8::splat(8.0) * t603 - f64x8::splat(25.0) / f64x8::splat(72.0) * t606;
            let t611 = t599 * t59;
            let t613 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t600 - f64x8::splat(27.0) / f64x8::splat(40.0) * t455 * t608 + t611 / f64x8::splat(36.0);
            let t622 = f64x8::splat(100.0) * t97 * v_sigma2 * t220 + f64x8::splat(324.0) * t589;
            let t625 = t96 * v_sigma2;
            let t629 = t115 * v_sigma2;
            let t632 = t122 * t170;
            let t636 = t593 * t189 / f64x8::splat(24.0) + t183 * t596 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t213 * t613 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t613 * t225 - f64x8::splat(73.0) / f64x8::splat(194400.0) * t473 * t622 + f64x8::splat(25.0) / f64x8::splat(472392.0) * t110 * t625 * t220 + t629 * t233 / f64x8::splat(360.0) + t120 * t632 * t239 / f64x8::splat(768.0);
            let t640 = t636 * t248 - t500 * t611 / f64x8::splat(12.0);
            let t641 = t416 * t640;
            let t642 = t280 * t641;
            let t645 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t414 * t642));
            let tvsigma2 = t7 * t645;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t646 = t6 * t28;
            let t647 = t279 * t282;
            let t648 = t29 * t46;
            let t653 = -t34 * t648 * t284 / f64x8::splat(8.0) - t38 * t31;
            let t654 = t653 * t51;
            let t655 = t40 * t654;
            let t656 = t45 * v_tau0;
            let t657 = f64x8::splat(1.0) / t656;
            let t658 = t44 * t657;
            let t662 = (t655 + t294 * t658 / f64x8::splat(16.0)) * t54;
            let t665 = t69 * t54;
            let t668 = param_b * t69;
            let t674 = f64x8::splat(5.0) * t668 * t80 + f64x8::splat(25.0) / f64x8::splat(9.0) * t78 * t97 * t69;
            let t677 = f64x8::splat(3.0) / f64x8::splat(4.0) * t665 * t316 - f64x8::splat(27.0) / f64x8::splat(40.0) * t321 * t674;
            let t684 = t43 * t657;
            let t687 = t662 * t65 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t90 * t677 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t677 * t106 + f64x8::splat(73.0) / f64x8::splat(600.0) * t339 * t658 - t116 * t684 / f64x8::splat(360.0);
            let t688 = t687 * t136;
            let t689 = t647 * t688;
            let t692 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t646 * t689));
            let tvtau0 = t7 * t692;
            acc_vtau_0 = tvtau0;
            let t693 = t6 * t157;
            let t694 = t279 * t416;
            let t695 = t158 * t175;
            let t700 = -t163 * t695 * t418 / f64x8::splat(8.0) - t167 * t160;
            let t701 = t700 * t180;
            let t702 = t169 * t701;
            let t703 = t174 * v_tau1;
            let t704 = f64x8::splat(1.0) / t703;
            let t705 = t173 * t704;
            let t709 = (t702 + t428 * t705 / f64x8::splat(16.0)) * t54;
            let t712 = t193 * t54;
            let t715 = param_b * t193;
            let t721 = f64x8::splat(5.0) * t715 * t203 + f64x8::splat(25.0) / f64x8::splat(9.0) * t202 * t97 * t193;
            let t724 = f64x8::splat(3.0) / f64x8::splat(4.0) * t712 * t450 - f64x8::splat(27.0) / f64x8::splat(40.0) * t455 * t721;
            let t731 = t172 * t704;
            let t734 = t709 * t189 / f64x8::splat(24.0) + f64x8::splat(292.0) / f64x8::splat(2025.0) * t213 * t724 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t724 * t225 + f64x8::splat(73.0) / f64x8::splat(600.0) * t473 * t705 - t232 * t731 / f64x8::splat(360.0);
            let t735 = t734 * t248;
            let t736 = t694 * t735;
            let t739 = ((t148).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t693 * t736));
            let tvtau1 = t7 * t739;
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
