//! GGA_X_PBEINT vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbeint.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_pbeint_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_muPBE: f64,
    param_muGE: f64,
    param_alpha: f64,
    param_kappa: f64,
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
        let t28 = param_muPBE - param_muGE;
        let t30 = M_CBRT6;
        let t31 = t28 * param_alpha * t30;
        let t32 = M_PI * M_PI;
        let t33 = pow_1_3(t32);
        let t34 = t33 * t33;
        let t35 = 1.0 / t34;
        let t36 = t35 * sigma0;
        let t37 = rho0 * rho0;
        let t38 = pow_1_3(rho0);
        let t39 = t38 * t38;
        let t41 = 1.0 / t39 / t37;
        let t42 = param_alpha * t30;
        let t43 = t36 * t41;
        let t46 = 1.0 + t42 * t43 / 24.0;
        let t47 = 1.0 / t46;
        let t53 = (param_muGE + t31 * t36 * t41 * t47 / 24.0) * t30;
        let t56 = param_kappa + t53 * t43 / 24.0;
        let t61 = 1.0 + param_kappa * (1.0 - param_kappa / t56);
        let t65 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t27 * t61);
        let t66 = rho1 <= dens_threshold;
        let t67 = -t16;
        let t69 = piecewise5(t14, t11, t10, t15, t67 * t7);
        let t70 = 1.0 + t69;
        let t71 = t70 <= zeta_threshold;
        let t72 = pow_1_3(t70);
        let t74 = piecewise3(t71, t22, t72 * t70);
        let t75 = t74 * t26;
        let t76 = t35 * sigma2;
        let t77 = rho1 * rho1;
        let t78 = pow_1_3(rho1);
        let t79 = t78 * t78;
        let t81 = 1.0 / t79 / t77;
        let t82 = t76 * t81;
        let t85 = 1.0 + t42 * t82 / 24.0;
        let t86 = 1.0 / t85;
        let t92 = (param_muGE + t31 * t76 * t81 * t86 / 24.0) * t30;
        let t95 = param_kappa + t92 * t82 / 24.0;
        let t100 = 1.0 + param_kappa * (1.0 - param_kappa / t95);
        let t104 = piecewise3(t66, 0.0, -3.0 / 8.0 * t5 * t75 * t100);
        let tzk0 = t65 + t104;
        zk[ip] += tzk0;
        let t105 = t6 * t6;
        let t106 = 1.0 / t105;
        let t107 = t16 * t106;
        let t109 = piecewise5(t10, 0.0, t14, 0.0, t7 - t107);
        let t112 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t109);
        let t113 = t112 * t26;
        let t117 = t26 * t26;
        let t118 = 1.0 / t117;
        let t119 = t25 * t118;
        let t122 = t5 * t119 * t61 / 8.0;
        let t123 = t5 * t25;
        let t124 = param_kappa * param_kappa;
        let t125 = t26 * t124;
        let t126 = t56 * t56;
        let t127 = 1.0 / t126;
        let t128 = t37 * rho0;
        let t130 = 1.0 / t39 / t128;
        let t135 = param_alpha * param_alpha;
        let t137 = t30 * t30;
        let t138 = t28 * t135 * t137;
        let t140 = 1.0 / t33 / t32;
        let t141 = sigma0 * sigma0;
        let t142 = t140 * t141;
        let t143 = t37 * t37;
        let t144 = t143 * t37;
        let t146 = 1.0 / t38 / t144;
        let t147 = t46 * t46;
        let t148 = 1.0 / t147;
        let t154 = (-t31 * t36 * t130 * t47 / 9.0 + t138 * t142 * t146 * t148 / 216.0) * t30;
        let t157 = t36 * t130;
        let t160 = t154 * t43 / 24.0 - t53 * t157 / 9.0;
        let t161 = t127 * t160;
        let t162 = t125 * t161;
        let t166 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t113 * t61 - t122 - 3.0 / 8.0 * t123 * t162);
        let t167 = t67 * t106;
        let t169 = piecewise5(t14, 0.0, t10, 0.0, -t7 - t167);
        let t172 = piecewise3(t71, 0.0, 4.0 / 3.0 * t72 * t169);
        let t173 = t172 * t26;
        let t177 = t74 * t118;
        let t180 = t5 * t177 * t100 / 8.0;
        let t182 = piecewise3(t66, 0.0, -3.0 / 8.0 * t5 * t173 * t100 - t180);
        let tvrho0 = t65 + t104 + t6 * (t166 + t182);
        vrho[ip * 2] += tvrho0;
        let t186 = piecewise5(t10, 0.0, t14, 0.0, -t7 - t107);
        let t189 = piecewise3(t20, 0.0, 4.0 / 3.0 * t23 * t186);
        let t190 = t189 * t26;
        let t195 = piecewise3(t1, 0.0, -3.0 / 8.0 * t5 * t190 * t61 - t122);
        let t197 = piecewise5(t14, 0.0, t10, 0.0, t7 - t167);
        let t200 = piecewise3(t71, 0.0, 4.0 / 3.0 * t72 * t197);
        let t201 = t200 * t26;
        let t205 = t5 * t74;
        let t206 = t95 * t95;
        let t207 = 1.0 / t206;
        let t208 = t77 * rho1;
        let t210 = 1.0 / t79 / t208;
        let t215 = sigma2 * sigma2;
        let t216 = t140 * t215;
        let t217 = t77 * t77;
        let t218 = t217 * t77;
        let t220 = 1.0 / t78 / t218;
        let t221 = t85 * t85;
        let t222 = 1.0 / t221;
        let t228 = (-t31 * t76 * t210 * t86 / 9.0 + t138 * t216 * t220 * t222 / 216.0) * t30;
        let t231 = t76 * t210;
        let t234 = t228 * t82 / 24.0 - t92 * t231 / 9.0;
        let t235 = t207 * t234;
        let t236 = t125 * t235;
        let t240 = piecewise3(t66, 0.0, -3.0 / 8.0 * t5 * t201 * t100 - t180 - 3.0 / 8.0 * t205 * t236);
        let tvrho1 = t65 + t104 + t6 * (t195 + t240);
        vrho[ip * 2 + 1] += tvrho1;
        let t243 = t35 * t41;
        let t248 = t143 * rho0;
        let t250 = 1.0 / t38 / t248;
        let t256 = (t31 * t243 * t47 / 24.0 - t138 * t140 * sigma0 * t250 * t148 / 576.0) * t30;
        let t260 = t53 * t243 / 24.0 + t256 * t43 / 24.0;
        let t261 = t127 * t260;
        let t262 = t125 * t261;
        let t265 = piecewise3(t1, 0.0, -3.0 / 8.0 * t123 * t262);
        let tvsigma0 = t6 * t265;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let t266 = t35 * t81;
        let t271 = t217 * rho1;
        let t273 = 1.0 / t78 / t271;
        let t279 = (t31 * t266 * t86 / 24.0 - t138 * t140 * sigma2 * t273 * t222 / 576.0) * t30;
        let t283 = t92 * t266 / 24.0 + t279 * t82 / 24.0;
        let t284 = t207 * t283;
        let t285 = t125 * t284;
        let t288 = piecewise3(t66, 0.0, -3.0 / 8.0 * t205 * t285);
        let tvsigma2 = t6 * t288;
        vsigma[ip * 3 + 2] += tvsigma2;
    }
}
