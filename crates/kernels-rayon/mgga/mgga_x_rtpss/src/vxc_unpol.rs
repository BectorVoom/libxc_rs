//! MGGA_X_RTPSS vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rtpss.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rtpss_vxc_unpol(
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
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = sigma[ip] * sigma[ip];
        let t22 = param_c * t21;
        let t23 = rho[ip] * rho[ip];
        let t24 = 1.0 / t23;
        let t25 = tau[ip] * tau[ip];
        let t26 = 1.0 / t25;
        let t27 = t24 * t26;
        let t28 = t21 * t24;
        let t29 = t28 * t26;
        let t31 = 1.0 + t29 / 64.0;
        let t32 = t31 * t31;
        let t33 = 1.0 / t32;
        let t34 = t27 * t33;
        let t38 = M_CBRT6;
        let t39 = (10.0 / 81.0 + t22 * t34 / 64.0) * t38;
        let t40 = M_PI * M_PI;
        let t41 = pow_1_3(t40);
        let t42 = t41 * t41;
        let t43 = 1.0 / t42;
        let t44 = t39 * t43;
        let t45 = M_CBRT2;
        let t46 = t45 * t45;
        let t47 = sigma[ip] * t46;
        let t48 = t19 * t19;
        let t50 = 1.0 / t48 / t23;
        let t51 = t47 * t50;
        let t54 = tau[ip] * t46;
        let t56 = 1.0 / t48 / rho[ip];
        let t59 = t54 * t56 - t51 / 8.0;
        let t63 = 5.0 / 9.0 * t59 * t38 * t43 - 1.0;
        let t64 = param_b * t59;
        let t65 = t38 * t43;
        let t66 = t65 * t63;
        let t69 = 5.0 * t64 * t66 + 9.0;
        let t70 = rmath::sqrt(t69);
        let t71 = 1.0 / t70;
        let t76 = 27.0 / 20.0 * t63 * t71 + t65 * t51 / 36.0;
        let t77 = t76 * t76;
        let t80 = t38 * t38;
        let t82 = 1.0 / t41 / t40;
        let t83 = t80 * t82;
        let t84 = t21 * t45;
        let t85 = t23 * t23;
        let t86 = t85 * rho[ip];
        let t88 = 1.0 / t19 / t86;
        let t89 = t84 * t88;
        let t92 = 100.0 * t83 * t89 + 162.0 * t29;
        let t93 = rmath::sqrt(t92);
        let t96 = 1.0 / param_kappa;
        let t97 = t96 * t80;
        let t98 = t97 * t82;
        let t101 = rmath::sqrt(param_e);
        let t102 = t101 * t21;
        let t105 = param_e * param_mu;
        let t106 = t40 * t40;
        let t107 = 1.0 / t106;
        let t108 = t21 * sigma[ip];
        let t109 = t107 * t108;
        let t110 = t85 * t85;
        let t111 = 1.0 / t110;
        let t115 = t44 * t51 / 24.0 + 146.0 / 2025.0 * t77 - 73.0 / 97200.0 * t76 * t93 + 25.0 / 472392.0 * t98 * t89 + t102 * t27 / 720.0 + t105 * t109 * t111 / 576.0;
        let t116 = t101 * t38;
        let t120 = 1.0 + t116 * t43 * t51 / 24.0;
        let t121 = t120 * t120;
        let t122 = 1.0 / t121;
        let t125 = rmath::exp(-t115 * t122 * t96);
        let t128 = 1.0 + param_kappa * (1.0 - t125);
        let t132 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t128);
        let tzk0 = 2.0 * t132;
        zk[ip] += tzk0;
        let t133 = 1.0 / t48;
        let t134 = t18 * t133;
        let t138 = t7 * t18;
        let t139 = t19 * param_kappa;
        let t140 = t23 * rho[ip];
        let t141 = 1.0 / t140;
        let t142 = t141 * t26;
        let t143 = t142 * t33;
        let t146 = t21 * t21;
        let t147 = param_c * t146;
        let t148 = 1.0 / t86;
        let t149 = t25 * t25;
        let t150 = 1.0 / t149;
        let t153 = 1.0 / t32 / t31;
        let t154 = t148 * t150 * t153;
        let t158 = (-t22 * t143 / 32.0 + t147 * t154 / 1024.0) * t38;
        let t159 = t158 * t43;
        let t163 = 1.0 / t48 / t140;
        let t164 = t47 * t163;
        let t170 = -5.0 / 3.0 * t54 * t50 + t164 / 3.0;
        let t171 = t170 * t38;
        let t172 = t43 * t71;
        let t176 = 1.0 / t70 / t69;
        let t177 = t63 * t176;
        let t181 = t83 * t170;
        let t184 = 5.0 * param_b * t170 * t66 + 25.0 / 9.0 * t64 * t181;
        let t187 = t65 * t164;
        let t189 = 3.0 / 4.0 * t171 * t172 - 27.0 / 40.0 * t177 * t184 - 2.0 / 27.0 * t187;
        let t194 = 1.0 / t93;
        let t195 = t76 * t194;
        let t196 = t21 * t141;
        let t199 = t85 * t23;
        let t201 = 1.0 / t19 / t199;
        let t202 = t84 * t201;
        let t205 = -324.0 * t196 * t26 - 1600.0 / 3.0 * t83 * t202;
        let t212 = t110 * rho[ip];
        let t213 = 1.0 / t212;
        let t217 = t159 * t51 / 24.0 - t44 * t164 / 9.0 + 292.0 / 2025.0 * t76 * t189 - 73.0 / 97200.0 * t189 * t93 - 73.0 / 194400.0 * t195 * t205 - 50.0 / 177147.0 * t98 * t202 - t102 * t142 / 360.0 - t105 * t109 * t213 / 72.0;
        let t220 = t121 * t120;
        let t221 = 1.0 / t220;
        let t223 = t96 * t101;
        let t224 = t115 * t221 * t223;
        let t227 = -t217 * t122 * t96 - 2.0 / 9.0 * t224 * t187;
        let t228 = t227 * t125;
        let t233 = piecewise3(t3, 0.0, -t7 * t134 * t128 / 8.0 + 3.0 / 8.0 * t138 * t139 * t228);
        let tvrho0 = 2.0 * rho[ip] * t233 + 2.0 * t132;
        vrho[ip] += tvrho0;
        let t236 = param_c * sigma[ip];
        let t239 = param_c * t108;
        let t240 = 1.0 / t85;
        let t241 = t240 * t150;
        let t242 = t241 * t153;
        let t246 = (t236 * t34 / 32.0 - t239 * t242 / 1024.0) * t38;
        let t247 = t246 * t43;
        let t250 = t43 * t46;
        let t251 = t250 * t50;
        let t254 = t46 * t50;
        let t255 = t65 * t71;
        let t256 = t254 * t255;
        let t258 = param_b * t46;
        let t259 = t258 * t50;
        let t260 = t259 * t66;
        let t262 = t64 * t80;
        let t263 = t82 * t46;
        let t265 = t262 * t263 * t50;
        let t267 = -5.0 / 8.0 * t260 - 25.0 / 72.0 * t265;
        let t270 = t254 * t65;
        let t272 = -3.0 / 32.0 * t256 - 27.0 / 40.0 * t177 * t267 + t270 / 36.0;
        let t277 = sigma[ip] * t24;
        let t280 = sigma[ip] * t45;
        let t281 = t280 * t88;
        let t284 = 324.0 * t277 * t26 + 200.0 * t83 * t281;
        let t289 = t101 * sigma[ip];
        let t292 = t107 * t21;
        let t296 = t247 * t51 / 24.0 + t39 * t251 / 24.0 + 292.0 / 2025.0 * t76 * t272 - 73.0 / 97200.0 * t272 * t93 - 73.0 / 194400.0 * t195 * t284 + 25.0 / 236196.0 * t98 * t281 + t289 * t27 / 360.0 + t105 * t292 * t111 / 192.0;
        let t301 = -t296 * t122 * t96 + t224 * t270 / 12.0;
        let t302 = t301 * t125;
        let t306 = piecewise3(t3, 0.0, 3.0 / 8.0 * t138 * t139 * t302);
        let tvsigma0 = 2.0 * rho[ip] * t306;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t308 = t25 * tau[ip];
        let t309 = 1.0 / t308;
        let t310 = t24 * t309;
        let t311 = t310 * t33;
        let t314 = t149 * tau[ip];
        let t315 = 1.0 / t314;
        let t317 = t240 * t315 * t153;
        let t321 = (-t22 * t311 / 32.0 + t147 * t317 / 1024.0) * t38;
        let t322 = t321 * t43;
        let t325 = t46 * t56;
        let t328 = t258 * t56;
        let t334 = 5.0 * t328 * t66 + 25.0 / 9.0 * t262 * t263 * t56;
        let t337 = 3.0 / 4.0 * t325 * t255 - 27.0 / 40.0 * t177 * t334;
        let t342 = t28 * t309;
        let t347 = t322 * t51 / 24.0 + 292.0 / 2025.0 * t76 * t337 - 73.0 / 97200.0 * t337 * t93 + 73.0 / 600.0 * t195 * t342 - t102 * t310 / 360.0;
        let t349 = t122 * t125;
        let t353 = piecewise3(t3, 0.0, -3.0 / 8.0 * t138 * t19 * t347 * t349);
        let tvtau0 = 2.0 * rho[ip] * t353;
        vtau[ip] += tvtau0;
    }
}
