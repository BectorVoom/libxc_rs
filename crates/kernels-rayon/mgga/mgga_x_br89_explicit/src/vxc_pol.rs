//! MGGA_X_BR89_EXPLICIT vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_br89_explicit.c`
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
pub fn mgga_x_br89_explicit_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
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
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t7 = (f64x8::splat(2.0) * v_rho0 * t4).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t11 = (f64x8::splat(2.0) * v_rho1 * t4).simd_le(zeta_threshold);
            let t12 = -t8;
            let t13 = v_rho0 - v_rho1;
            let t15 = ((t7).select(t8, (t11).select(t12, t13 * t4)));
            let t16 = f64x8::splat(1.0) + t15;
            let t17 = (t16).simd_le(zeta_threshold);
            let t18 = (simd::cbrt(zeta_threshold));
            let t19 = t18 * zeta_threshold;
            let t20 = (simd::cbrt(t16));
            let t22 = ((t17).select(t19, t20 * t16));
            let t23 = (simd::cbrt(t3));
            let t24 = t22 * t23;
            let t26 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t27 = f64x8::splat(1.0) / t26;
            let t28 = t24 * t27;
            let t29 = f64x8::splat(M_CBRT4);
            let t30 = f64x8::splat(M_CBRTPI);
            let t31 = t30 * t30;
            let t32 = (simd::cbrt(v_rho0));
            let t33 = t32 * t32;
            let t35 = f64x8::splat(1.0) / t33 / v_rho0;
            let t36 = v_lapl0 * t35;
            let t38 = v_tau0 * param_gamma;
            let t39 = t38 * t35;
            let t41 = param_gamma * v_sigma0;
            let t42 = v_rho0 * v_rho0;
            let t44 = f64x8::splat(1.0) / t33 / t42;
            let t45 = t41 * t44;
            let t48 = ((t36 / f64x8::splat(2.0) - f64x8::splat(2.0) * t39 + t45 / f64x8::splat(4.0)).abs());
            let t50 = (t48 / f64x8::splat(3.0)).simd_lt(f64x8::splat(5e-13));
            let t54 = t36 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t39 + t45 / f64x8::splat(12.0);
            let t55 = (f64x8::splat(0.0)).simd_lt(t54);
            let t56 = ((t55).select(f64x8::splat(5e-13), -f64x8::splat(5e-13)));
            let t57 = ((t50).select(t56, t54));
            let t60 = f64x8::splat(2.0) / f64x8::splat(3.0) * t31 / t57;
            let t61 = (t60).simd_le(f64x8::splat(0.0));
            let t62 = (-f64x8::splat(5e-13)).simd_lt(t60);
            let t63 = ((t62).select(-f64x8::splat(5e-13), t60));
            let t65 = f64x8::splat(1.525525181200953) * t63 + f64x8::splat(0.4576575543602858);
            let t66 = (simd::atan(t65));
            let t67 = -t66 + f64x8::splat(0.4292036732051034);
            let t69 = t63 * t63;
            let t71 = t69 * t63;
            let t73 = t69 * t69;
            let t75 = t73 * t63;
            let t77 = f64x8::splat(0.7566445420735584) - f64x8::splat(2.636397787137096) * t63 + f64x8::splat(5.474515996423288) * t69 - f64x8::splat(12.65730812710829) * t71 + f64x8::splat(4.125058472512136) * t73 - f64x8::splat(30.42513395716384) * t75;
            let t78 = t67 * t77;
            let t84 = f64x8::splat(0.4771976183772063) - f64x8::splat(1.779981349455627) * t63 + f64x8::splat(3.843384186230215) * t69 - f64x8::splat(9.591205088051849) * t71 + f64x8::splat(2.173018028591672) * t73 - f64x8::splat(30.42513385160366) * t75;
            let t85 = f64x8::splat(1.0) / t84;
            let t87 = (f64x8::splat(5e-13)).simd_lt(t60);
            let t88 = ((t87).select(t60, f64x8::splat(5e-13)));
            let t90 = (simd::ln(f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t88) + ((((f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t88)) * (f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t88))) + f64x8::splat(1.0)).sqrt())));
            let t91 = t90 + f64x8::splat(2.0);
            let t93 = t88 * t88;
            let t95 = t93 * t88;
            let t97 = t93 * t93;
            let t99 = t97 * t88;
            let t101 = f64x8::splat(4.435009886795587e-05) + f64x8::splat(0.5812865360445791) * t88 + f64x8::splat(66.7427645159406) * t93 + f64x8::splat(434.2678089722977) * t95 + f64x8::splat(824.7765766052239) * t97 + f64x8::splat(1657.965273158212) * t99;
            let t102 = t91 * t101;
            let t108 = f64x8::splat(3.347285060926091e-05) + f64x8::splat(0.4791793102397135) * t88 + f64x8::splat(62.39226833857424) * t93 + f64x8::splat(463.1481642793812) * t95 + f64x8::splat(785.2360350104029) * t97 + f64x8::splat(1657.962968223273) * t99;
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = ((t61).select(t78 * t85, t102 * t109));
            let t113 = (simd::exp(t111 / f64x8::splat(3.0)));
            let t114 = t29 * t113;
            let t115 = (simd::exp(-t111));
            let t117 = f64x8::splat(1.0) + t111 / f64x8::splat(2.0);
            let t118 = t115 * t117;
            let t119 = f64x8::splat(1.0) - t118;
            let t120 = f64x8::splat(1.0) / t111;
            let t121 = t119 * t120;
            let t122 = t114 * t121;
            let t125 = ((t2).select(f64x8::splat(0.0), -t28 * t122 / f64x8::splat(4.0)));
            let t126 = (v_rho1).simd_le(dens_threshold);
            let t127 = -t13;
            let t129 = ((t11).select(t8, (t7).select(t12, t127 * t4)));
            let t130 = f64x8::splat(1.0) + t129;
            let t131 = (t130).simd_le(zeta_threshold);
            let t132 = (simd::cbrt(t130));
            let t134 = ((t131).select(t19, t132 * t130));
            let t135 = t134 * t23;
            let t136 = t135 * t27;
            let t137 = (simd::cbrt(v_rho1));
            let t138 = t137 * t137;
            let t140 = f64x8::splat(1.0) / t138 / v_rho1;
            let t141 = v_lapl1 * t140;
            let t143 = v_tau1 * param_gamma;
            let t144 = t143 * t140;
            let t146 = param_gamma * v_sigma2;
            let t147 = v_rho1 * v_rho1;
            let t149 = f64x8::splat(1.0) / t138 / t147;
            let t150 = t146 * t149;
            let t153 = ((t141 / f64x8::splat(2.0) - f64x8::splat(2.0) * t144 + t150 / f64x8::splat(4.0)).abs());
            let t155 = (t153 / f64x8::splat(3.0)).simd_lt(f64x8::splat(5e-13));
            let t159 = t141 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t144 + t150 / f64x8::splat(12.0);
            let t160 = (f64x8::splat(0.0)).simd_lt(t159);
            let t161 = ((t160).select(f64x8::splat(5e-13), -f64x8::splat(5e-13)));
            let t162 = ((t155).select(t161, t159));
            let t165 = f64x8::splat(2.0) / f64x8::splat(3.0) * t31 / t162;
            let t166 = (t165).simd_le(f64x8::splat(0.0));
            let t167 = (-f64x8::splat(5e-13)).simd_lt(t165);
            let t168 = ((t167).select(-f64x8::splat(5e-13), t165));
            let t170 = f64x8::splat(1.525525181200953) * t168 + f64x8::splat(0.4576575543602858);
            let t171 = (simd::atan(t170));
            let t172 = -t171 + f64x8::splat(0.4292036732051034);
            let t174 = t168 * t168;
            let t176 = t174 * t168;
            let t178 = t174 * t174;
            let t180 = t178 * t168;
            let t182 = f64x8::splat(0.7566445420735584) - f64x8::splat(2.636397787137096) * t168 + f64x8::splat(5.474515996423288) * t174 - f64x8::splat(12.65730812710829) * t176 + f64x8::splat(4.125058472512136) * t178 - f64x8::splat(30.42513395716384) * t180;
            let t183 = t172 * t182;
            let t189 = f64x8::splat(0.4771976183772063) - f64x8::splat(1.779981349455627) * t168 + f64x8::splat(3.843384186230215) * t174 - f64x8::splat(9.591205088051849) * t176 + f64x8::splat(2.173018028591672) * t178 - f64x8::splat(30.42513385160366) * t180;
            let t190 = f64x8::splat(1.0) / t189;
            let t192 = (f64x8::splat(5e-13)).simd_lt(t165);
            let t193 = ((t192).select(t165, f64x8::splat(5e-13)));
            let t195 = (simd::ln(f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t193) + ((((f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t193)) * (f64x8::splat(1.0) / (f64x8::splat(2.085749716493756) * t193))) + f64x8::splat(1.0)).sqrt())));
            let t196 = t195 + f64x8::splat(2.0);
            let t198 = t193 * t193;
            let t200 = t198 * t193;
            let t202 = t198 * t198;
            let t204 = t202 * t193;
            let t206 = f64x8::splat(4.435009886795587e-05) + f64x8::splat(0.5812865360445791) * t193 + f64x8::splat(66.7427645159406) * t198 + f64x8::splat(434.2678089722977) * t200 + f64x8::splat(824.7765766052239) * t202 + f64x8::splat(1657.965273158212) * t204;
            let t207 = t196 * t206;
            let t213 = f64x8::splat(3.347285060926091e-05) + f64x8::splat(0.4791793102397135) * t193 + f64x8::splat(62.39226833857424) * t198 + f64x8::splat(463.1481642793812) * t200 + f64x8::splat(785.2360350104029) * t202 + f64x8::splat(1657.962968223273) * t204;
            let t214 = f64x8::splat(1.0) / t213;
            let t216 = ((t166).select(t183 * t190, t207 * t214));
            let t218 = (simd::exp(t216 / f64x8::splat(3.0)));
            let t219 = t29 * t218;
            let t220 = (simd::exp(-t216));
            let t222 = f64x8::splat(1.0) + t216 / f64x8::splat(2.0);
            let t223 = t220 * t222;
            let t224 = f64x8::splat(1.0) - t223;
            let t225 = f64x8::splat(1.0) / t216;
            let t226 = t224 * t225;
            let t227 = t219 * t226;
            let t230 = ((t126).select(f64x8::splat(0.0), -t136 * t227 / f64x8::splat(4.0)));
            let tzk0 = t125 + t230;
            acc_zk = tzk0;
            let t231 = t3 * t3;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t13 * t232;
            let t235 = ((t7).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t4 - t233)));
            let t238 = ((t17).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t20 * t235));
            let t239 = t238 * t23;
            let t240 = t239 * t27;
            let t243 = t23 * t23;
            let t244 = f64x8::splat(1.0) / t243;
            let t245 = t22 * t244;
            let t246 = t245 * t27;
            let t248 = t246 * t122 / f64x8::splat(12.0);
            let t249 = t27 * t29;
            let t250 = t24 * t249;
            let t251 = t57 * t57;
            let t253 = t31 / t251;
            let t254 = ((t55).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t261 = f64x8::splat(1.0) / t33 / t42 / v_rho0;
            let t265 = ((t50).select(t254, -f64x8::splat(5.0) / f64x8::splat(18.0) * v_lapl0 * t44 + f64x8::splat(10.0) / f64x8::splat(9.0) * t38 * t44 - f64x8::splat(2.0) / f64x8::splat(9.0) * t41 * t261));
            let t267 = f64x8::splat(2.0) / f64x8::splat(3.0) * t253 * t265;
            let t268 = ((t62).select(f64x8::splat(0.0), -t267));
            let t269 = t65 * t65;
            let t270 = t269 + f64x8::splat(1.0);
            let t271 = f64x8::splat(1.0) / t270;
            let t272 = t268 * t271;
            let t273 = t77 * t85;
            let t277 = t63 * t268;
            let t279 = t69 * t268;
            let t281 = t71 * t268;
            let t283 = t73 * t268;
            let t285 = -f64x8::splat(2.636397787137096) * t268 + f64x8::splat(10.949031992846576) * t277 - f64x8::splat(37.97192438132487) * t279 + f64x8::splat(16.500233890048545) * t281 - f64x8::splat(152.1256697858192) * t283;
            let t286 = t67 * t285;
            let t288 = t84 * t84;
            let t289 = f64x8::splat(1.0) / t288;
            let t295 = -f64x8::splat(1.779981349455627) * t268 + f64x8::splat(7.68676837246043) * t277 - f64x8::splat(28.77361526415555) * t279 + f64x8::splat(8.692072114366688) * t281 - f64x8::splat(152.1256692580183) * t283;
            let t296 = t289 * t295;
            let t299 = ((t87).select(-t267, f64x8::splat(0.0)));
            let t300 = f64x8::splat(1.0) / t93;
            let t301 = t299 * t300;
            let t303 = f64x8::splat(1.0) + f64x8::splat(0.2298664631316238) * t300;
            let t304 = ((t303).sqrt());
            let t305 = f64x8::splat(1.0) / t304;
            let t306 = t305 * t101;
            let t307 = t306 * t109;
            let t311 = t88 * t299;
            let t313 = t93 * t299;
            let t315 = t95 * t299;
            let t317 = t97 * t299;
            let t319 = f64x8::splat(0.5812865360445791) * t299 + f64x8::splat(133.4855290318812) * t311 + f64x8::splat(1302.8034269168932) * t313 + f64x8::splat(3299.1063064208956) * t315 + f64x8::splat(8289.82636579106) * t317;
            let t320 = t91 * t319;
            let t322 = t108 * t108;
            let t323 = f64x8::splat(1.0) / t322;
            let t329 = f64x8::splat(0.4791793102397135) * t299 + f64x8::splat(124.78453667714848) * t311 + f64x8::splat(1389.4444928381436) * t313 + f64x8::splat(3140.9441400416117) * t315 + f64x8::splat(8289.814841116366) * t317;
            let t330 = t323 * t329;
            let t333 = ((t61).select(-f64x8::splat(1.525525181200953) * t272 * t273 + t286 * t85 - t78 * t296, -f64x8::splat(0.47944391030820677) * t301 * t307 + t320 * t109 - t102 * t330));
            let t334 = t333 * t113;
            let t335 = t334 * t121;
            let t338 = t333 * t115;
            let t339 = t338 * t117;
            let t341 = t339 - t338 / f64x8::splat(2.0);
            let t342 = t341 * t120;
            let t343 = t114 * t342;
            let t346 = t113 * t119;
            let t347 = t111 * t111;
            let t348 = f64x8::splat(1.0) / t347;
            let t349 = t348 * t333;
            let t350 = t346 * t349;
            let t354 = ((t2).select(f64x8::splat(0.0), -t240 * t122 / f64x8::splat(4.0) - t248 - t250 * t335 / f64x8::splat(12.0) - t28 * t343 / f64x8::splat(4.0) + t250 * t350 / f64x8::splat(4.0)));
            let t355 = t127 * t232;
            let t357 = ((t11).select(f64x8::splat(0.0), (t7).select(f64x8::splat(0.0), -t4 - t355)));
            let t360 = ((t131).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t132 * t357));
            let t361 = t360 * t23;
            let t362 = t361 * t27;
            let t365 = t134 * t244;
            let t366 = t365 * t27;
            let t368 = t366 * t227 / f64x8::splat(12.0);
            let t370 = ((t126).select(f64x8::splat(0.0), -t362 * t227 / f64x8::splat(4.0) - t368));
            let tvrho0 = t125 + t230 + t3 * (t354 + t370);
            acc_vrho_0 = tvrho0;
            let t374 = ((t7).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t4 - t233)));
            let t377 = ((t17).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t20 * t374));
            let t378 = t377 * t23;
            let t379 = t378 * t27;
            let t383 = ((t2).select(f64x8::splat(0.0), -t379 * t122 / f64x8::splat(4.0) - t248));
            let t385 = ((t11).select(f64x8::splat(0.0), (t7).select(f64x8::splat(0.0), t4 - t355)));
            let t388 = ((t131).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t132 * t385));
            let t389 = t388 * t23;
            let t390 = t389 * t27;
            let t393 = t135 * t249;
            let t394 = t162 * t162;
            let t396 = t31 / t394;
            let t397 = ((t160).select(f64x8::splat(0.0), f64x8::splat(0.0)));
            let t404 = f64x8::splat(1.0) / t138 / t147 / v_rho1;
            let t408 = ((t155).select(t397, -f64x8::splat(5.0) / f64x8::splat(18.0) * v_lapl1 * t149 + f64x8::splat(10.0) / f64x8::splat(9.0) * t143 * t149 - f64x8::splat(2.0) / f64x8::splat(9.0) * t146 * t404));
            let t410 = f64x8::splat(2.0) / f64x8::splat(3.0) * t396 * t408;
            let t411 = ((t167).select(f64x8::splat(0.0), -t410));
            let t412 = t170 * t170;
            let t413 = t412 + f64x8::splat(1.0);
            let t414 = f64x8::splat(1.0) / t413;
            let t415 = t411 * t414;
            let t416 = t182 * t190;
            let t420 = t168 * t411;
            let t422 = t174 * t411;
            let t424 = t176 * t411;
            let t426 = t178 * t411;
            let t428 = -f64x8::splat(2.636397787137096) * t411 + f64x8::splat(10.949031992846576) * t420 - f64x8::splat(37.97192438132487) * t422 + f64x8::splat(16.500233890048545) * t424 - f64x8::splat(152.1256697858192) * t426;
            let t429 = t172 * t428;
            let t431 = t189 * t189;
            let t432 = f64x8::splat(1.0) / t431;
            let t438 = -f64x8::splat(1.779981349455627) * t411 + f64x8::splat(7.68676837246043) * t420 - f64x8::splat(28.77361526415555) * t422 + f64x8::splat(8.692072114366688) * t424 - f64x8::splat(152.1256692580183) * t426;
            let t439 = t432 * t438;
            let t442 = ((t192).select(-t410, f64x8::splat(0.0)));
            let t443 = f64x8::splat(1.0) / t198;
            let t444 = t442 * t443;
            let t446 = f64x8::splat(1.0) + f64x8::splat(0.2298664631316238) * t443;
            let t447 = ((t446).sqrt());
            let t448 = f64x8::splat(1.0) / t447;
            let t449 = t448 * t206;
            let t450 = t449 * t214;
            let t454 = t193 * t442;
            let t456 = t198 * t442;
            let t458 = t200 * t442;
            let t460 = t202 * t442;
            let t462 = f64x8::splat(0.5812865360445791) * t442 + f64x8::splat(133.4855290318812) * t454 + f64x8::splat(1302.8034269168932) * t456 + f64x8::splat(3299.1063064208956) * t458 + f64x8::splat(8289.82636579106) * t460;
            let t463 = t196 * t462;
            let t465 = t213 * t213;
            let t466 = f64x8::splat(1.0) / t465;
            let t472 = f64x8::splat(0.4791793102397135) * t442 + f64x8::splat(124.78453667714848) * t454 + f64x8::splat(1389.4444928381436) * t456 + f64x8::splat(3140.9441400416117) * t458 + f64x8::splat(8289.814841116366) * t460;
            let t473 = t466 * t472;
            let t476 = ((t166).select(-f64x8::splat(1.525525181200953) * t415 * t416 + t429 * t190 - t183 * t439, -f64x8::splat(0.47944391030820677) * t444 * t450 + t463 * t214 - t207 * t473));
            let t477 = t476 * t218;
            let t478 = t477 * t226;
            let t481 = t476 * t220;
            let t482 = t481 * t222;
            let t484 = t482 - t481 / f64x8::splat(2.0);
            let t485 = t484 * t225;
            let t486 = t219 * t485;
            let t489 = t218 * t224;
            let t490 = t216 * t216;
            let t491 = f64x8::splat(1.0) / t490;
            let t492 = t491 * t476;
            let t493 = t489 * t492;
            let t497 = ((t126).select(f64x8::splat(0.0), -t390 * t227 / f64x8::splat(4.0) - t368 - t393 * t478 / f64x8::splat(12.0) - t136 * t486 / f64x8::splat(4.0) + t393 * t493 / f64x8::splat(4.0)));
            let tvrho1 = t125 + t230 + t3 * (t383 + t497);
            acc_vrho_1 = tvrho1;
            let t500 = param_gamma * t44;
            let t502 = ((t50).select(t254, t500 / f64x8::splat(12.0)));
            let t504 = f64x8::splat(2.0) / f64x8::splat(3.0) * t253 * t502;
            let t505 = ((t62).select(f64x8::splat(0.0), -t504));
            let t506 = t505 * t271;
            let t510 = t63 * t505;
            let t512 = t69 * t505;
            let t514 = t71 * t505;
            let t516 = t73 * t505;
            let t518 = -f64x8::splat(2.636397787137096) * t505 + f64x8::splat(10.949031992846576) * t510 - f64x8::splat(37.97192438132487) * t512 + f64x8::splat(16.500233890048545) * t514 - f64x8::splat(152.1256697858192) * t516;
            let t519 = t67 * t518;
            let t526 = -f64x8::splat(1.779981349455627) * t505 + f64x8::splat(7.68676837246043) * t510 - f64x8::splat(28.77361526415555) * t512 + f64x8::splat(8.692072114366688) * t514 - f64x8::splat(152.1256692580183) * t516;
            let t527 = t289 * t526;
            let t530 = ((t87).select(-t504, f64x8::splat(0.0)));
            let t531 = t530 * t300;
            let t535 = t88 * t530;
            let t537 = t93 * t530;
            let t539 = t95 * t530;
            let t541 = t97 * t530;
            let t543 = f64x8::splat(0.5812865360445791) * t530 + f64x8::splat(133.4855290318812) * t535 + f64x8::splat(1302.8034269168932) * t537 + f64x8::splat(3299.1063064208956) * t539 + f64x8::splat(8289.82636579106) * t541;
            let t544 = t91 * t543;
            let t551 = f64x8::splat(0.4791793102397135) * t530 + f64x8::splat(124.78453667714848) * t535 + f64x8::splat(1389.4444928381436) * t537 + f64x8::splat(3140.9441400416117) * t539 + f64x8::splat(8289.814841116366) * t541;
            let t552 = t323 * t551;
            let t555 = ((t61).select(-f64x8::splat(1.525525181200953) * t506 * t273 + t519 * t85 - t78 * t527, -f64x8::splat(0.47944391030820677) * t531 * t307 + t544 * t109 - t102 * t552));
            let t556 = t555 * t113;
            let t557 = t556 * t121;
            let t560 = t555 * t115;
            let t561 = t560 * t117;
            let t563 = t561 - t560 / f64x8::splat(2.0);
            let t564 = t563 * t120;
            let t565 = t114 * t564;
            let t568 = t348 * t555;
            let t569 = t346 * t568;
            let t573 = ((t2).select(f64x8::splat(0.0), -t250 * t557 / f64x8::splat(12.0) - t28 * t565 / f64x8::splat(4.0) + t250 * t569 / f64x8::splat(4.0)));
            let tvsigma0 = t3 * t573;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t574 = param_gamma * t149;
            let t576 = ((t155).select(t397, t574 / f64x8::splat(12.0)));
            let t578 = f64x8::splat(2.0) / f64x8::splat(3.0) * t396 * t576;
            let t579 = ((t167).select(f64x8::splat(0.0), -t578));
            let t580 = t579 * t414;
            let t584 = t168 * t579;
            let t586 = t174 * t579;
            let t588 = t176 * t579;
            let t590 = t178 * t579;
            let t592 = -f64x8::splat(2.636397787137096) * t579 + f64x8::splat(10.949031992846576) * t584 - f64x8::splat(37.97192438132487) * t586 + f64x8::splat(16.500233890048545) * t588 - f64x8::splat(152.1256697858192) * t590;
            let t593 = t172 * t592;
            let t600 = -f64x8::splat(1.779981349455627) * t579 + f64x8::splat(7.68676837246043) * t584 - f64x8::splat(28.77361526415555) * t586 + f64x8::splat(8.692072114366688) * t588 - f64x8::splat(152.1256692580183) * t590;
            let t601 = t432 * t600;
            let t604 = ((t192).select(-t578, f64x8::splat(0.0)));
            let t605 = t604 * t443;
            let t609 = t193 * t604;
            let t611 = t198 * t604;
            let t613 = t200 * t604;
            let t615 = t202 * t604;
            let t617 = f64x8::splat(0.5812865360445791) * t604 + f64x8::splat(133.4855290318812) * t609 + f64x8::splat(1302.8034269168932) * t611 + f64x8::splat(3299.1063064208956) * t613 + f64x8::splat(8289.82636579106) * t615;
            let t618 = t196 * t617;
            let t625 = f64x8::splat(0.4791793102397135) * t604 + f64x8::splat(124.78453667714848) * t609 + f64x8::splat(1389.4444928381436) * t611 + f64x8::splat(3140.9441400416117) * t613 + f64x8::splat(8289.814841116366) * t615;
            let t626 = t466 * t625;
            let t629 = ((t166).select(-f64x8::splat(1.525525181200953) * t580 * t416 + t593 * t190 - t183 * t601, -f64x8::splat(0.47944391030820677) * t605 * t450 + t618 * t214 - t207 * t626));
            let t630 = t629 * t218;
            let t631 = t630 * t226;
            let t634 = t629 * t220;
            let t635 = t634 * t222;
            let t637 = t635 - t634 / f64x8::splat(2.0);
            let t638 = t637 * t225;
            let t639 = t219 * t638;
            let t642 = t491 * t629;
            let t643 = t489 * t642;
            let t647 = ((t126).select(f64x8::splat(0.0), -t393 * t631 / f64x8::splat(12.0) - t136 * t639 / f64x8::splat(4.0) + t393 * t643 / f64x8::splat(4.0)));
            let tvsigma2 = t3 * t647;
            acc_vsigma_2 = tvsigma2;
            let t649 = ((t50).select(t254, t35 / f64x8::splat(6.0)));
            let t651 = f64x8::splat(2.0) / f64x8::splat(3.0) * t253 * t649;
            let t652 = ((t62).select(f64x8::splat(0.0), -t651));
            let t653 = t652 * t271;
            let t657 = t63 * t652;
            let t659 = t69 * t652;
            let t661 = t71 * t652;
            let t663 = t73 * t652;
            let t665 = -f64x8::splat(2.636397787137096) * t652 + f64x8::splat(10.949031992846576) * t657 - f64x8::splat(37.97192438132487) * t659 + f64x8::splat(16.500233890048545) * t661 - f64x8::splat(152.1256697858192) * t663;
            let t666 = t67 * t665;
            let t673 = -f64x8::splat(1.779981349455627) * t652 + f64x8::splat(7.68676837246043) * t657 - f64x8::splat(28.77361526415555) * t659 + f64x8::splat(8.692072114366688) * t661 - f64x8::splat(152.1256692580183) * t663;
            let t674 = t289 * t673;
            let t677 = ((t87).select(-t651, f64x8::splat(0.0)));
            let t678 = t677 * t300;
            let t682 = t88 * t677;
            let t684 = t93 * t677;
            let t686 = t95 * t677;
            let t688 = t97 * t677;
            let t690 = f64x8::splat(0.5812865360445791) * t677 + f64x8::splat(133.4855290318812) * t682 + f64x8::splat(1302.8034269168932) * t684 + f64x8::splat(3299.1063064208956) * t686 + f64x8::splat(8289.82636579106) * t688;
            let t691 = t91 * t690;
            let t698 = f64x8::splat(0.4791793102397135) * t677 + f64x8::splat(124.78453667714848) * t682 + f64x8::splat(1389.4444928381436) * t684 + f64x8::splat(3140.9441400416117) * t686 + f64x8::splat(8289.814841116366) * t688;
            let t699 = t323 * t698;
            let t702 = ((t61).select(-f64x8::splat(1.525525181200953) * t653 * t273 + t666 * t85 - t78 * t674, -f64x8::splat(0.47944391030820677) * t678 * t307 + t691 * t109 - t102 * t699));
            let t703 = t702 * t113;
            let t704 = t703 * t121;
            let t707 = t702 * t115;
            let t708 = t707 * t117;
            let t710 = t708 - t707 / f64x8::splat(2.0);
            let t711 = t710 * t120;
            let t712 = t114 * t711;
            let t715 = t348 * t702;
            let t716 = t346 * t715;
            let t720 = ((t2).select(f64x8::splat(0.0), -t250 * t704 / f64x8::splat(12.0) - t28 * t712 / f64x8::splat(4.0) + t250 * t716 / f64x8::splat(4.0)));
            let tvlapl0 = t3 * t720;
            acc_vlapl_0 = tvlapl0;
            let t722 = ((t155).select(t397, t140 / f64x8::splat(6.0)));
            let t724 = f64x8::splat(2.0) / f64x8::splat(3.0) * t396 * t722;
            let t725 = ((t167).select(f64x8::splat(0.0), -t724));
            let t726 = t725 * t414;
            let t730 = t168 * t725;
            let t732 = t174 * t725;
            let t734 = t176 * t725;
            let t736 = t178 * t725;
            let t738 = -f64x8::splat(2.636397787137096) * t725 + f64x8::splat(10.949031992846576) * t730 - f64x8::splat(37.97192438132487) * t732 + f64x8::splat(16.500233890048545) * t734 - f64x8::splat(152.1256697858192) * t736;
            let t739 = t172 * t738;
            let t746 = -f64x8::splat(1.779981349455627) * t725 + f64x8::splat(7.68676837246043) * t730 - f64x8::splat(28.77361526415555) * t732 + f64x8::splat(8.692072114366688) * t734 - f64x8::splat(152.1256692580183) * t736;
            let t747 = t432 * t746;
            let t750 = ((t192).select(-t724, f64x8::splat(0.0)));
            let t751 = t750 * t443;
            let t755 = t193 * t750;
            let t757 = t198 * t750;
            let t759 = t200 * t750;
            let t761 = t202 * t750;
            let t763 = f64x8::splat(0.5812865360445791) * t750 + f64x8::splat(133.4855290318812) * t755 + f64x8::splat(1302.8034269168932) * t757 + f64x8::splat(3299.1063064208956) * t759 + f64x8::splat(8289.82636579106) * t761;
            let t764 = t196 * t763;
            let t771 = f64x8::splat(0.4791793102397135) * t750 + f64x8::splat(124.78453667714848) * t755 + f64x8::splat(1389.4444928381436) * t757 + f64x8::splat(3140.9441400416117) * t759 + f64x8::splat(8289.814841116366) * t761;
            let t772 = t466 * t771;
            let t775 = ((t166).select(-f64x8::splat(1.525525181200953) * t726 * t416 + t739 * t190 - t183 * t747, -f64x8::splat(0.47944391030820677) * t751 * t450 + t764 * t214 - t207 * t772));
            let t776 = t775 * t218;
            let t777 = t776 * t226;
            let t780 = t775 * t220;
            let t781 = t780 * t222;
            let t783 = t781 - t780 / f64x8::splat(2.0);
            let t784 = t783 * t225;
            let t785 = t219 * t784;
            let t788 = t491 * t775;
            let t789 = t489 * t788;
            let t793 = ((t126).select(f64x8::splat(0.0), -t393 * t777 / f64x8::splat(12.0) - t136 * t785 / f64x8::splat(4.0) + t393 * t789 / f64x8::splat(4.0)));
            let tvlapl1 = t3 * t793;
            acc_vlapl_1 = tvlapl1;
            let t796 = ((t50).select(t254, -f64x8::splat(2.0) / f64x8::splat(3.0) * param_gamma * t35));
            let t798 = f64x8::splat(2.0) / f64x8::splat(3.0) * t253 * t796;
            let t799 = ((t62).select(f64x8::splat(0.0), -t798));
            let t800 = t799 * t271;
            let t804 = t63 * t799;
            let t806 = t69 * t799;
            let t808 = t71 * t799;
            let t810 = t73 * t799;
            let t812 = -f64x8::splat(2.636397787137096) * t799 + f64x8::splat(10.949031992846576) * t804 - f64x8::splat(37.97192438132487) * t806 + f64x8::splat(16.500233890048545) * t808 - f64x8::splat(152.1256697858192) * t810;
            let t813 = t67 * t812;
            let t820 = -f64x8::splat(1.779981349455627) * t799 + f64x8::splat(7.68676837246043) * t804 - f64x8::splat(28.77361526415555) * t806 + f64x8::splat(8.692072114366688) * t808 - f64x8::splat(152.1256692580183) * t810;
            let t821 = t289 * t820;
            let t824 = ((t87).select(-t798, f64x8::splat(0.0)));
            let t825 = t824 * t300;
            let t829 = t88 * t824;
            let t831 = t93 * t824;
            let t833 = t95 * t824;
            let t835 = t97 * t824;
            let t837 = f64x8::splat(0.5812865360445791) * t824 + f64x8::splat(133.4855290318812) * t829 + f64x8::splat(1302.8034269168932) * t831 + f64x8::splat(3299.1063064208956) * t833 + f64x8::splat(8289.82636579106) * t835;
            let t838 = t91 * t837;
            let t845 = f64x8::splat(0.4791793102397135) * t824 + f64x8::splat(124.78453667714848) * t829 + f64x8::splat(1389.4444928381436) * t831 + f64x8::splat(3140.9441400416117) * t833 + f64x8::splat(8289.814841116366) * t835;
            let t846 = t323 * t845;
            let t849 = ((t61).select(-f64x8::splat(1.525525181200953) * t800 * t273 + t813 * t85 - t78 * t821, -f64x8::splat(0.47944391030820677) * t825 * t307 + t838 * t109 - t102 * t846));
            let t850 = t849 * t113;
            let t851 = t850 * t121;
            let t854 = t849 * t115;
            let t857 = t854 * t117 - t854 / f64x8::splat(2.0);
            let t858 = t857 * t120;
            let t859 = t114 * t858;
            let t862 = t348 * t849;
            let t863 = t346 * t862;
            let t867 = ((t2).select(f64x8::splat(0.0), -t250 * t851 / f64x8::splat(12.0) - t28 * t859 / f64x8::splat(4.0) + t250 * t863 / f64x8::splat(4.0)));
            let tvtau0 = t3 * t867;
            acc_vtau_0 = tvtau0;
            let t870 = ((t155).select(t397, -f64x8::splat(2.0) / f64x8::splat(3.0) * param_gamma * t140));
            let t872 = f64x8::splat(2.0) / f64x8::splat(3.0) * t396 * t870;
            let t873 = ((t167).select(f64x8::splat(0.0), -t872));
            let t874 = t873 * t414;
            let t878 = t168 * t873;
            let t880 = t174 * t873;
            let t882 = t176 * t873;
            let t884 = t178 * t873;
            let t886 = -f64x8::splat(2.636397787137096) * t873 + f64x8::splat(10.949031992846576) * t878 - f64x8::splat(37.97192438132487) * t880 + f64x8::splat(16.500233890048545) * t882 - f64x8::splat(152.1256697858192) * t884;
            let t887 = t172 * t886;
            let t894 = -f64x8::splat(1.779981349455627) * t873 + f64x8::splat(7.68676837246043) * t878 - f64x8::splat(28.77361526415555) * t880 + f64x8::splat(8.692072114366688) * t882 - f64x8::splat(152.1256692580183) * t884;
            let t895 = t432 * t894;
            let t898 = ((t192).select(-t872, f64x8::splat(0.0)));
            let t899 = t898 * t443;
            let t903 = t193 * t898;
            let t905 = t198 * t898;
            let t907 = t200 * t898;
            let t909 = t202 * t898;
            let t911 = f64x8::splat(0.5812865360445791) * t898 + f64x8::splat(133.4855290318812) * t903 + f64x8::splat(1302.8034269168932) * t905 + f64x8::splat(3299.1063064208956) * t907 + f64x8::splat(8289.82636579106) * t909;
            let t912 = t196 * t911;
            let t919 = f64x8::splat(0.4791793102397135) * t898 + f64x8::splat(124.78453667714848) * t903 + f64x8::splat(1389.4444928381436) * t905 + f64x8::splat(3140.9441400416117) * t907 + f64x8::splat(8289.814841116366) * t909;
            let t920 = t466 * t919;
            let t923 = ((t166).select(-f64x8::splat(1.525525181200953) * t874 * t416 + t887 * t190 - t183 * t895, -f64x8::splat(0.47944391030820677) * t899 * t450 + t912 * t214 - t207 * t920));
            let t924 = t923 * t218;
            let t925 = t924 * t226;
            let t928 = t923 * t220;
            let t931 = t928 * t222 - t928 / f64x8::splat(2.0);
            let t932 = t931 * t225;
            let t933 = t219 * t932;
            let t936 = t491 * t923;
            let t937 = t489 * t936;
            let t941 = ((t126).select(f64x8::splat(0.0), -t393 * t925 / f64x8::splat(12.0) - t136 * t933 / f64x8::splat(4.0) + t393 * t937 / f64x8::splat(4.0)));
            let tvtau1 = t3 * t941;
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
