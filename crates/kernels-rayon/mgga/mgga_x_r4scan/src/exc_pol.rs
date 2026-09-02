//! MGGA_X_R4SCAN exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r4scan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_r4scan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_da4: f64,
    param_dp2: f64,
    param_dp4: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = rho0 + rho1;
        let t8 = 1.0 / t7;
        let t11 = 2.0 * rho0 * t8 <= zeta_threshold;
        let t12 = zeta_threshold - 1.0;
        let t15 = 2.0 * rho1 * t8 <= zeta_threshold;
        let t16 = -t12;
        let t17 = rho0 - rho1;
        let t19 = piecewise5(t11, t12, t15, t16, t17 * t8);
        let t20 = 1.0 + t19;
        let t21 = t20 <= zeta_threshold;
        let t22 = pow_1_3(zeta_threshold);
        let t23 = t22 * zeta_threshold;
        let t24 = pow_1_3(t20);
        let t26 = piecewise3(t21, t23, t24 * t20);
        let t27 = t6 * t26;
        let t28 = pow_1_3(t7);
        let t30 = 20.0 / 27.0 + 5.0 / 3.0 * param_eta;
        let t31 = M_CBRT6;
        let t32 = t31 * t31;
        let t33 = M_PI * M_PI;
        let t34 = pow_1_3(t33);
        let t35 = t34 * t33;
        let t36 = 1.0 / t35;
        let t37 = t32 * t36;
        let t38 = sigma0 * sigma0;
        let t39 = rho0 * rho0;
        let t40 = t39 * t39;
        let t41 = t40 * rho0;
        let t42 = pow_1_3(rho0);
        let t44 = 1.0 / t42 / t41;
        let t45 = t38 * t44;
        let t46 = param_dp2 * param_dp2;
        let t47 = t46 * t46;
        let t48 = 1.0 / t47;
        let t52 = rmath::exp(-t37 * t45 * t48 / 576.0);
        let t56 = (-0.162742215233874 * t30 * t52 + 10.0 / 81.0) * t31;
        let t57 = t34 * t34;
        let t58 = 1.0 / t57;
        let t59 = t58 * sigma0;
        let t60 = t42 * t42;
        let t61 = t60 * t39;
        let t62 = 1.0 / t61;
        let t63 = t59 * t62;
        let t66 = param_k1 + t56 * t63 / 24.0;
        let t70 = param_k1 * (1.0 - param_k1 / t66);
        let t71 = t60 * rho0;
        let t72 = 1.0 / t71;
        let t74 = sigma0 * t62;
        let t76 = tau0 * t72 - t74 / 8.0;
        let t78 = 3.0 / 10.0 * t32 * t57;
        let t79 = param_eta * sigma0;
        let t82 = t78 + t79 * t62 / 8.0;
        let t83 = 1.0 / t82;
        let t84 = t76 * t83;
        let t85 = t84 <= 0.0;
        let t86 = 0.0 < t84;
        let t87 = piecewise3(t86, 0.0, t84);
        let t88 = param_c1 * t87;
        let t89 = 1.0 - t87;
        let t90 = 1.0 / t89;
        let t92 = rmath::exp(-t88 * t90);
        let t93 = t84 <= 2.5;
        let t94 = 2.5 < t84;
        let t95 = piecewise3(t94, 2.5, t84);
        let t97 = t95 * t95;
        let t99 = t97 * t95;
        let t101 = t97 * t97;
        let t103 = t101 * t95;
        let t105 = t101 * t97;
        let t110 = piecewise3(t94, t84, 2.5);
        let t111 = 1.0 - t110;
        let t114 = rmath::exp(param_c2 / t111);
        let t116 = piecewise5(t85, t92, t93, 1.0 - 0.667 * t95 - 0.4445555 * t97 - 0.663086601049 * t99 + 1.45129704449 * t101 - 0.887998041597 * t103 + 0.234528941479 * t105 - 0.023185843322 * t101 * t99, -param_d * t114);
        let t117 = 0.174 - t70;
        let t120 = t30 * t31;
        let t123 = 1.0 - t84;
        let t124 = t123 * t123;
        let t128 = (0.040570770199022686 - 0.3023546802608101 * param_eta) * t31;
        let t129 = t128 * t58;
        let t135 = pow_2(3.0 / 4.0 * param_eta + 2.0 / 3.0);
        let t140 = pow_2(0.0029070010613279013 - 0.27123702538979 * param_eta);
        let t144 = (146.0 / 2025.0 * t135 - 73.0 / 540.0 * param_eta - 146.0 / 1215.0 + t140 / param_k1) * t32;
        let t145 = t36 * t38;
        let t149 = -0.162742215233874 + 0.162742215233874 * t84 + 0.00678092563474475 * t120 * t63 - 0.059353125082804 * t124 + t129 * t74 * t123 / 24.0 + t144 * t145 * t44 / 576.0;
        let t150 = t76 * t76;
        let t151 = t149 * t150;
        let t152 = t82 * t82;
        let t153 = 1.0 / t152;
        let t154 = t150 * t150;
        let t155 = t152 * t152;
        let t156 = 1.0 / t155;
        let t158 = t154 * t156 + 1.0;
        let t159 = 1.0 / t158;
        let t160 = t153 * t159;
        let t161 = param_da4 * param_da4;
        let t162 = 1.0 / t161;
        let t164 = param_dp4 * param_dp4;
        let t165 = t164 * t164;
        let t166 = 1.0 / t165;
        let t171 = rmath::exp(-t124 * t162 - t37 * t45 * t166 / 576.0);
        let t172 = t160 * t171;
        let t175 = t116 * t117 + 2.0 * t151 * t172 + t70 + 1.0;
        let t176 = t28 * t175;
        let t177 = rmath::sqrt(3.0);
        let t178 = 1.0 / t34;
        let t179 = t32 * t178;
        let t180 = rmath::sqrt(sigma0);
        let t181 = t42 * rho0;
        let t182 = 1.0 / t181;
        let t184 = t179 * t180 * t182;
        let t185 = rmath::sqrt(t184);
        let t189 = rmath::exp(-9.8958 * t177 / t185);
        let t190 = 1.0 - t189;
        let t191 = t176 * t190;
        let t194 = piecewise3(t2, 0.0, -3.0 / 8.0 * t27 * t191);
        let t195 = rho1 <= dens_threshold;
        let t196 = -t17;
        let t198 = piecewise5(t15, t12, t11, t16, t196 * t8);
        let t199 = 1.0 + t198;
        let t200 = t199 <= zeta_threshold;
        let t201 = pow_1_3(t199);
        let t203 = piecewise3(t200, t23, t201 * t199);
        let t204 = t6 * t203;
        let t205 = sigma2 * sigma2;
        let t206 = rho1 * rho1;
        let t207 = t206 * t206;
        let t208 = t207 * rho1;
        let t209 = pow_1_3(rho1);
        let t211 = 1.0 / t209 / t208;
        let t212 = t205 * t211;
        let t216 = rmath::exp(-t37 * t212 * t48 / 576.0);
        let t220 = (-0.162742215233874 * t30 * t216 + 10.0 / 81.0) * t31;
        let t221 = t58 * sigma2;
        let t222 = t209 * t209;
        let t223 = t222 * t206;
        let t224 = 1.0 / t223;
        let t225 = t221 * t224;
        let t228 = param_k1 + t220 * t225 / 24.0;
        let t232 = param_k1 * (1.0 - param_k1 / t228);
        let t233 = t222 * rho1;
        let t234 = 1.0 / t233;
        let t236 = sigma2 * t224;
        let t238 = tau1 * t234 - t236 / 8.0;
        let t239 = param_eta * sigma2;
        let t242 = t78 + t239 * t224 / 8.0;
        let t243 = 1.0 / t242;
        let t244 = t238 * t243;
        let t245 = t244 <= 0.0;
        let t246 = 0.0 < t244;
        let t247 = piecewise3(t246, 0.0, t244);
        let t248 = param_c1 * t247;
        let t249 = 1.0 - t247;
        let t250 = 1.0 / t249;
        let t252 = rmath::exp(-t248 * t250);
        let t253 = t244 <= 2.5;
        let t254 = 2.5 < t244;
        let t255 = piecewise3(t254, 2.5, t244);
        let t257 = t255 * t255;
        let t259 = t257 * t255;
        let t261 = t257 * t257;
        let t263 = t261 * t255;
        let t265 = t261 * t257;
        let t270 = piecewise3(t254, t244, 2.5);
        let t271 = 1.0 - t270;
        let t274 = rmath::exp(param_c2 / t271);
        let t276 = piecewise5(t245, t252, t253, 1.0 - 0.667 * t255 - 0.4445555 * t257 - 0.663086601049 * t259 + 1.45129704449 * t261 - 0.887998041597 * t263 + 0.234528941479 * t265 - 0.023185843322 * t261 * t259, -param_d * t274);
        let t277 = 0.174 - t232;
        let t282 = 1.0 - t244;
        let t283 = t282 * t282;
        let t288 = t36 * t205;
        let t292 = -0.162742215233874 + 0.162742215233874 * t244 + 0.00678092563474475 * t120 * t225 - 0.059353125082804 * t283 + t129 * t236 * t282 / 24.0 + t144 * t288 * t211 / 576.0;
        let t293 = t238 * t238;
        let t294 = t292 * t293;
        let t295 = t242 * t242;
        let t296 = 1.0 / t295;
        let t297 = t293 * t293;
        let t298 = t295 * t295;
        let t299 = 1.0 / t298;
        let t301 = t297 * t299 + 1.0;
        let t302 = 1.0 / t301;
        let t303 = t296 * t302;
        let t309 = rmath::exp(-t283 * t162 - t37 * t212 * t166 / 576.0);
        let t310 = t303 * t309;
        let t313 = t276 * t277 + 2.0 * t294 * t310 + t232 + 1.0;
        let t314 = t28 * t313;
        let t315 = rmath::sqrt(sigma2);
        let t316 = t209 * rho1;
        let t317 = 1.0 / t316;
        let t319 = t179 * t315 * t317;
        let t320 = rmath::sqrt(t319);
        let t324 = rmath::exp(-9.8958 * t177 / t320);
        let t325 = 1.0 - t324;
        let t326 = t314 * t325;
        let t329 = piecewise3(t195, 0.0, -3.0 / 8.0 * t204 * t326);
        let tzk0 = t194 + t329;
        zk[ip] += tzk0;
    }
}
