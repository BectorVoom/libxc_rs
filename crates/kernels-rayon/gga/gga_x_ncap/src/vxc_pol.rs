//! GGA_X_NCAP vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ncap.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ncap_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_mu: f64,
    param_zeta: f64,
    param_alpha: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let t1 = rho0 <= dens_threshold;
        let t2 = M_CBRT3;
        let t3 = M_CBRTPI;
        let t5 = t2 / t3;
        let t6 = rho0 + rho1;
        let t7 = 1.0 / t6;
        let t10 = 2.0 * rho0 * t7 <= zeta_threshold;
        let t11 = zeta_threshold - 1.0;
        let t14 = 2.0 * rho1 * t7 <= zeta_threshold;
        let t15 = -t11;
        let t16 = rho0 - rho1;
        let t18 = piecewise5(t10, t11, t14, t15, t16 * t7);
        let t19 = 1.0 + t18;
        let t20 = t19 <= zeta_threshold;
        let t21 = pow_1_3(zeta_threshold);
        let t22 = t21 * zeta_threshold;
        let t23 = pow_1_3(t19);
        let t25 = piecewise3(t20, t22, t23 * t19);
        let t26 = pow_1_3(t6);
        let t27 = t25 * t26;
        let t28 = M_CBRT6;
        let t29 = t28 * t28;
        let t30 = M_PI * M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = t29 * t32;
        let t34 = f64::sqrt(sigma0);
        let t35 = pow_1_3(rho0);
        let t37 = 1.0 / t35 / rho0;
        let t38 = t34 * t37;
        let t40 = t33 * t38 / 12.0;
        let t41 = f64::tanh(t40);
        let t42 = param_mu * t41;
        let t43 = f64::ln(t40 + f64::sqrt(t40 * t40 + 1.0));
        let t44 = 1.0 - param_zeta;
        let t46 = t44 * t29 * t32;
        let t47 = 1.0 + t40;
        let t48 = f64::ln(t47);
        let t51 = param_zeta * t29;
        let t52 = t32 * t34;
        let t58 = 1.0 + param_alpha * (t51 * t52 * t37 / 12.0 + t46 * t38 * t48 / 12.0);
        let t59 = t43 * t58;
        let t60 = param_beta * t41;
        let t62 = t60 * t43 + 1.0;
        let t63 = 1.0 / t62;
        let t64 = t59 * t63;
        let t66 = t42 * t64 + 1.0;
        let t70 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t66);
        let t71 = rho1 <= dens_threshold;
        let t72 = -t16;
        let t74 = piecewise5(t14, t11, t10, t15, t72 * t7);
        let t75 = 1.0 + t74;
        let t76 = t75 <= zeta_threshold;
        let t77 = pow_1_3(t75);
        let t79 = piecewise3(t76, t22, t77 * t75);
        let t80 = t79 * t26;
        let t81 = f64::sqrt(sigma2);
        let t82 = pow_1_3(rho1);
        let t84 = 1.0 / t82 / rho1;
        let t85 = t81 * t84;
        let t87 = t33 * t85 / 12.0;
        let t88 = f64::tanh(t87);
        let t89 = param_mu * t88;
        let t90 = f64::ln(t87 + f64::sqrt(t87 * t87 + 1.0));
        let t91 = 1.0 + t87;
        let t92 = f64::ln(t91);
        let t95 = t32 * t81;
        let t101 = 1.0 + param_alpha * (t46 * t85 * t92 / 12.0 + t51 * t95 * t84 / 12.0);
        let t102 = t90 * t101;
        let t103 = param_beta * t88;
        let t105 = t103 * t90 + 1.0;
        let t106 = 1.0 / t105;
        let t107 = t102 * t106;
        let t109 = t89 * t107 + 1.0;
        let t113 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t80 * t109);
        let tzk0 = t70 + t113;
        zk[ip] += tzk0;
        let t114 = t6 * t6;
        let t115 = 1.0 / t114;
        let t116 = t16 * t115;
        let t118 = piecewise5(t10, 0.0, t14, 0.0, t7 - t116);
        let t121 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t118);
        let t122 = t121 * t26;
        let t126 = t26 * t26;
        let t127 = 1.0 / t126;
        let t128 = t25 * t127;
        let t131 = t5 * t128 * t66 / 8.0;
        let t132 = param_mu * t29;
        let t133 = t132 * t52;
        let t134 = rho0 * rho0;
        let t136 = 1.0 / t35 / t134;
        let t137 = t41 * t41;
        let t138 = 1.0 - t137;
        let t140 = t136 * t138 * t64;
        let t143 = t42 * t33;
        let t144 = t34 * t136;
        let t145 = t31 * t31;
        let t146 = 1.0 / t145;
        let t147 = t28 * t146;
        let t148 = t35 * t35;
        let t150 = 1.0 / t148 / t134;
        let t154 = 6.0 * t147 * sigma0 * t150 + 144.0;
        let t155 = f64::sqrt(t154);
        let t156 = 1.0 / t155;
        let t157 = t156 * t58;
        let t158 = t157 * t63;
        let t162 = t42 * t43;
        let t166 = t44 * t28;
        let t167 = t166 * t146;
        let t168 = t134 * rho0;
        let t170 = 1.0 / t148 / t168;
        let t172 = 1.0 / t47;
        let t176 = t52 * t136;
        let t179 = -t46 * t144 * t48 / 9.0 - t167 * sigma0 * t170 * t172 / 18.0 - t51 * t176 / 9.0;
        let t180 = param_alpha * t179;
        let t181 = t180 * t63;
        let t183 = t62 * t62;
        let t184 = 1.0 / t183;
        let t185 = t58 * t184;
        let t186 = param_beta * t29;
        let t187 = t186 * t32;
        let t188 = t138 * t43;
        let t192 = t60 * t29;
        let t193 = t136 * t156;
        let t197 = -t187 * t144 * t188 / 9.0 - 4.0 / 3.0 * t192 * t52 * t193;
        let t198 = t185 * t197;
        let t200 = -t133 * t140 / 9.0 - 4.0 / 3.0 * t143 * t144 * t158 + t162 * t181 - t162 * t198;
        let t205 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t122 * t66 - t131 - 3.0 / 8.0 * t5 * t27 * t200);
        let t206 = t72 * t115;
        let t208 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t206);
        let t211 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t208);
        let t212 = t211 * t26;
        let t216 = t79 * t127;
        let t219 = t5 * t216 * t109 / 8.0;
        let t221 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t212 * t109 - t219);
        let tvrho0 = t70 + t113 + t6 * (t205 + t221);
        vrho[ip * 2] += tvrho0;
        let t225 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t116);
        let t228 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t225);
        let t229 = t228 * t26;
        let t234 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t229 * t66 - t131);
        let t236 = piecewise5(t14, 0.0, t10, 0.0, t7 - t206);
        let t239 = piecewise3(t76, 0.0, 4.0 / 3.0 * t77 * t236);
        let t240 = t239 * t26;
        let t244 = t132 * t95;
        let t245 = rho1 * rho1;
        let t247 = 1.0 / t82 / t245;
        let t248 = t88 * t88;
        let t249 = 1.0 - t248;
        let t251 = t247 * t249 * t107;
        let t254 = t89 * t33;
        let t255 = t81 * t247;
        let t256 = t82 * t82;
        let t258 = 1.0 / t256 / t245;
        let t262 = 6.0 * t147 * sigma2 * t258 + 144.0;
        let t263 = f64::sqrt(t262);
        let t264 = 1.0 / t263;
        let t265 = t264 * t101;
        let t266 = t265 * t106;
        let t270 = t89 * t90;
        let t274 = t245 * rho1;
        let t276 = 1.0 / t256 / t274;
        let t278 = 1.0 / t91;
        let t282 = t95 * t247;
        let t285 = -t46 * t255 * t92 / 9.0 - t167 * sigma2 * t276 * t278 / 18.0 - t51 * t282 / 9.0;
        let t286 = param_alpha * t285;
        let t287 = t286 * t106;
        let t289 = t105 * t105;
        let t290 = 1.0 / t289;
        let t291 = t101 * t290;
        let t292 = t249 * t90;
        let t296 = t103 * t29;
        let t297 = t247 * t264;
        let t301 = -t187 * t255 * t292 / 9.0 - 4.0 / 3.0 * t296 * t95 * t297;
        let t302 = t291 * t301;
        let t304 = -t244 * t251 / 9.0 - 4.0 / 3.0 * t254 * t255 * t266 + t270 * t287 - t270 * t302;
        let t309 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t240 * t109 - t219 - 3.0 / 8.0 * t5 * t80 * t304);
        let tvrho1 = t70 + t113 + t6 * (t234 + t309);
        vrho[ip * 2 + 1] += tvrho1;
        let t312 = 1.0 / t34;
        let t313 = t32 * t312;
        let t314 = t132 * t313;
        let t316 = t37 * t138 * t64;
        let t319 = t312 * t37;
        let t330 = t313 * t37;
        let t333 = t46 * t319 * t48 / 24.0 + t166 * t146 * t150 * t172 / 48.0 + t51 * t330 / 24.0;
        let t334 = param_alpha * t333;
        let t335 = t334 * t63;
        let t340 = t37 * t156;
        let t344 = t187 * t319 * t188 / 24.0 + t192 * t313 * t340 / 2.0;
        let t345 = t185 * t344;
        let t347 = t314 * t316 / 24.0 + t143 * t319 * t158 / 2.0 + t162 * t335 - t162 * t345;
        let t351 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t347);
        let tvsigma0 = t6 * t351;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t352 = 1.0 / t81;
        let t353 = t32 * t352;
        let t354 = t132 * t353;
        let t356 = t84 * t249 * t107;
        let t359 = t352 * t84;
        let t370 = t353 * t84;
        let t373 = t46 * t359 * t92 / 24.0 + t166 * t146 * t258 * t278 / 48.0 + t51 * t370 / 24.0;
        let t374 = param_alpha * t373;
        let t375 = t374 * t106;
        let t380 = t84 * t264;
        let t384 = t187 * t359 * t292 / 24.0 + t296 * t353 * t380 / 2.0;
        let t385 = t291 * t384;
        let t387 = t354 * t356 / 24.0 + t254 * t359 * t266 / 2.0 + t270 * t375 - t270 * t385;
        let t391 = piecewise3(t71, 0.0, -3.0 / 8.0 * t5 * t80 * t387);
        let tvsigma2 = t6 * t391;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
