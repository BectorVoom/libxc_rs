//! MGGA_X_REGTM exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_regtm_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let t27 = pow_1_3(t7);
        let t28 = t26 * t27;
        let t29 = pow_1_3(rho0);
        let t30 = t29 * t29;
        let t32 = 1.0 / t30 / rho0;
        let t33 = tau0 * t32;
        let t34 = rho0 * rho0;
        let t36 = 1.0 / t30 / t34;
        let t37 = sigma0 * t36;
        let t39 = t33 - t37 / 8.0;
        let t40 = M_CBRT6;
        let t41 = t39 * t40;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t40 * t45;
        let t47 = t46 * t37;
        let t49 = t41 * t45;
        let t51 = 1.0 - 5.0 / 9.0 * t49;
        let t52 = t51 * t51;
        let t53 = t52 * t51;
        let t54 = t39 * t39;
        let t55 = t40 * t40;
        let t56 = t54 * t55;
        let t58 = 1.0 / t43 / t42;
        let t61 = 1.0 + 0.6714891975308642 * t56 * t58;
        let t62 = rmath::sqrt(t61);
        let t64 = 1.0 / t62 / t61;
        let t65 = t53 * t64;
        let t67 = rmath::exp(-t47 / 8.0);
        let t69 = t47 / 24.0 + t65 * t67;
        let t71 = t45 / t69;
        let t74 = 1.0 + t41 * t71 / 3.0;
        let t75 = t74 * t74;
        let t77 = t75 * t74;
        let t78 = 1.0 / t77;
        let t80 = 1.0 / t75 + 3.0 * t78;
        let t81 = 1.0 + t78;
        let t82 = t81 * t81;
        let t83 = 1.0 / t82;
        let t84 = t80 * t83;
        let t86 = t55 * t58;
        let t87 = sigma0 * sigma0;
        let t88 = t34 * t34;
        let t89 = t88 * rho0;
        let t91 = 1.0 / t29 / t89;
        let t95 = 1.0 + 0.1504548888888889 * t47 + 0.002689949046226295 * t86 * t87 * t91;
        let t96 = rmath::pow(t95, 1.0 / 5.0);
        let t101 = 0.256337604 * t55 * t44;
        let t107 = 1.0 + 0.06394332777777778 * t47 - 5.0 / 9.0 * (0.14554132 * t33 + t101 + 0.011867481666666667 * t37) * t40 * t45;
        let t108 = t96 * t96;
        let t109 = 1.0 / t108;
        let t112 = 1.0 / t96 + 7.0 / 9.0 * t107 * t109;
        let t114 = 1.0 - t84;
        let t117 = (10.0 / 81.0 + 25.0 / 8748.0 * t47) * t40;
        let t118 = t45 * sigma0;
        let t124 = t49 / 4.0 - 9.0 / 20.0 + t47 / 36.0;
        let t125 = t124 * t124;
        let t127 = 1.0 / rho0;
        let t128 = sigma0 * t127;
        let t129 = 1.0 / tau0;
        let t131 = t128 * t129 / 8.0;
        let t132 = t131 < 1.0;
        let t133 = piecewise3(t132, t131, 1.0);
        let t134 = t124 * t133;
        let t135 = 1.0 - t133;
        let t138 = 1.0 + 5.0 / 12.0 * t117 * t118 * t36 + 292.0 / 405.0 * t125 - 146.0 / 135.0 * t134 * t135;
        let t139 = rmath::pow(t138, 1.0 / 10.0);
        let t141 = t84 * t112 + t114 * t139;
        let t145 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t141);
        let t146 = rho1 <= dens_threshold;
        let t147 = -t17;
        let t149 = piecewise5(t15, t12, t11, t16, t147 * t8);
        let t150 = 1.0 + t149;
        let t151 = t150 <= zeta_threshold;
        let t152 = pow_1_3(t150);
        let t154 = piecewise3(t151, t23, t152 * t150);
        let t155 = t154 * t27;
        let t156 = pow_1_3(rho1);
        let t157 = t156 * t156;
        let t159 = 1.0 / t157 / rho1;
        let t160 = tau1 * t159;
        let t161 = rho1 * rho1;
        let t163 = 1.0 / t157 / t161;
        let t164 = sigma2 * t163;
        let t166 = t160 - t164 / 8.0;
        let t167 = t166 * t40;
        let t168 = t46 * t164;
        let t170 = t167 * t45;
        let t172 = 1.0 - 5.0 / 9.0 * t170;
        let t173 = t172 * t172;
        let t174 = t173 * t172;
        let t175 = t166 * t166;
        let t176 = t175 * t55;
        let t179 = 1.0 + 0.6714891975308642 * t176 * t58;
        let t180 = rmath::sqrt(t179);
        let t182 = 1.0 / t180 / t179;
        let t183 = t174 * t182;
        let t185 = rmath::exp(-t168 / 8.0);
        let t187 = t168 / 24.0 + t183 * t185;
        let t189 = t45 / t187;
        let t192 = 1.0 + t167 * t189 / 3.0;
        let t193 = t192 * t192;
        let t195 = t193 * t192;
        let t196 = 1.0 / t195;
        let t198 = 1.0 / t193 + 3.0 * t196;
        let t199 = 1.0 + t196;
        let t200 = t199 * t199;
        let t201 = 1.0 / t200;
        let t202 = t198 * t201;
        let t204 = sigma2 * sigma2;
        let t205 = t161 * t161;
        let t206 = t205 * rho1;
        let t208 = 1.0 / t156 / t206;
        let t212 = 1.0 + 0.1504548888888889 * t168 + 0.002689949046226295 * t86 * t204 * t208;
        let t213 = rmath::pow(t212, 1.0 / 5.0);
        let t222 = 1.0 + 0.06394332777777778 * t168 - 5.0 / 9.0 * (0.14554132 * t160 + t101 + 0.011867481666666667 * t164) * t40 * t45;
        let t223 = t213 * t213;
        let t224 = 1.0 / t223;
        let t227 = 1.0 / t213 + 7.0 / 9.0 * t222 * t224;
        let t229 = 1.0 - t202;
        let t232 = (10.0 / 81.0 + 25.0 / 8748.0 * t168) * t40;
        let t233 = t45 * sigma2;
        let t239 = t170 / 4.0 - 9.0 / 20.0 + t168 / 36.0;
        let t240 = t239 * t239;
        let t242 = 1.0 / rho1;
        let t243 = sigma2 * t242;
        let t244 = 1.0 / tau1;
        let t246 = t243 * t244 / 8.0;
        let t247 = t246 < 1.0;
        let t248 = piecewise3(t247, t246, 1.0);
        let t249 = t239 * t248;
        let t250 = 1.0 - t248;
        let t253 = 1.0 + 5.0 / 12.0 * t232 * t233 * t163 + 292.0 / 405.0 * t240 - 146.0 / 135.0 * t249 * t250;
        let t254 = rmath::pow(t253, 1.0 / 10.0);
        let t256 = t202 * t227 + t229 * t254;
        let t260 = piecewise3(t146, 0.0, -3.0 / 8.0 * t6 * t155 * t256);
        let tzk0 = t145 + t260;
        zk[ip] += tzk0;
    }
}
