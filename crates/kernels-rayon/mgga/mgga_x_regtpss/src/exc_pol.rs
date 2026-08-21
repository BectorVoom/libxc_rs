//! MGGA_X_REGTPSS exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtpss.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_regtpss_exc_pol(
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
        let t29 = 1.0 / rho0;
        let t30 = sigma0 * t29;
        let t31 = 1.0 / tau0;
        let t32 = t30 * t31;
        let t33 = pow_3(t32);
        let t34 = sigma0 * sigma0;
        let t35 = rho0 * rho0;
        let t36 = 1.0 / t35;
        let t37 = t34 * t36;
        let t38 = tau0 * tau0;
        let t39 = 1.0 / t38;
        let t40 = t37 * t39;
        let t42 = 1.0 + t40 / 64.0;
        let t43 = t42 * t42;
        let t44 = 1.0 / t43;
        let t48 = M_CBRT6;
        let t49 = (10.0 / 81.0 + 0.0045938270703125 * t33 * t44) * t48;
        let t50 = M_PI * M_PI;
        let t51 = pow_1_3(t50);
        let t52 = t51 * t51;
        let t53 = 1.0 / t52;
        let t54 = t53 * sigma0;
        let t55 = pow_1_3(rho0);
        let t56 = t55 * t55;
        let t58 = 1.0 / t56 / t35;
        let t59 = t54 * t58;
        let t63 = 1.0 / t56 / rho0;
        let t65 = sigma0 * t58;
        let t67 = tau0 * t63 - t65 / 8.0;
        let t68 = t67 * t48;
        let t69 = t68 * t53;
        let t71 = 5.0 / 9.0 * t69 - 1.0;
        let t72 = t53 * t71;
        let t75 = 1.0 + 0.2222222222222222 * t68 * t72;
        let t76 = rmath::sqrt(t75);
        let t77 = 1.0 / t76;
        let t80 = t48 * t53;
        let t81 = t80 * t65;
        let t82 = t81 / 36.0;
        let t83 = 9.0 / 20.0 * t71 * t77 + t82;
        let t84 = t83 * t83;
        let t87 = t48 * t48;
        let t89 = 1.0 / t51 / t50;
        let t90 = t87 * t89;
        let t91 = t35 * t35;
        let t92 = t91 * rho0;
        let t94 = 1.0 / t55 / t92;
        let t96 = t90 * t34 * t94;
        let t97 = 50.0 * t96;
        let t98 = 162.0 * t40 + t97;
        let t99 = rmath::sqrt(t98);
        let t102 = 3.291178445357254e-05 * t96;
        let t104 = t34 * sigma0;
        let t105 = t91 * t91;
        let t106 = 1.0 / t105;
        let t108 = 1.3522126526770064e-06 * t104 * t106;
        let t109 = t49 * t59 / 24.0 + 146.0 / 2025.0 * t84 - 73.0 / 97200.0 * t83 * t99 + t102 + 0.0020448759451792767 * t40 + t108;
        let t111 = 1.0 + 0.06134627835537829 * t81;
        let t112 = t111 * t111;
        let t113 = 1.0 / t112;
        let t115 = 0.804 + t109 * t113;
        let t117 = 0.646416 / t115;
        let t118 = -t71;
        let t119 = t118 * t118;
        let t120 = t119 * t118;
        let t121 = t67 * t67;
        let t122 = t121 * t87;
        let t123 = t122 * t89;
        let t125 = 1.0 + 0.6714891975308642 * t123;
        let t126 = rmath::sqrt(t125);
        let t128 = 1.0 / t126 / t125;
        let t129 = t120 * t128;
        let t131 = rmath::exp(-t81 / 8.0);
        let t133 = -0.45 + t82;
        let t134 = t133 * t133;
        let t136 = 10368.0 + t97;
        let t137 = rmath::sqrt(t136);
        let t140 = 0.029644443963477367 * t81 + 146.0 / 2025.0 * t134 - 73.0 / 97200.0 * t133 * t137 + t102 + 0.1308720604914737 + t108;
        let t142 = 0.804 + t140 * t113;
        let t145 = -0.646416 / t142 + t117;
        let t146 = t131 * t145;
        let t148 = 1.804 - t117 + t129 * t146;
        let t152 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t148);
        let t153 = rho1 <= dens_threshold;
        let t154 = -t17;
        let t156 = piecewise5(t15, t12, t11, t16, t154 * t8);
        let t157 = 1.0 + t156;
        let t158 = t157 <= zeta_threshold;
        let t159 = pow_1_3(t157);
        let t161 = piecewise3(t158, t23, t159 * t157);
        let t162 = t161 * t27;
        let t163 = 1.0 / rho1;
        let t164 = sigma2 * t163;
        let t165 = 1.0 / tau1;
        let t166 = t164 * t165;
        let t167 = pow_3(t166);
        let t168 = sigma2 * sigma2;
        let t169 = rho1 * rho1;
        let t170 = 1.0 / t169;
        let t171 = t168 * t170;
        let t172 = tau1 * tau1;
        let t173 = 1.0 / t172;
        let t174 = t171 * t173;
        let t176 = 1.0 + t174 / 64.0;
        let t177 = t176 * t176;
        let t178 = 1.0 / t177;
        let t182 = (10.0 / 81.0 + 0.0045938270703125 * t167 * t178) * t48;
        let t183 = t53 * sigma2;
        let t184 = pow_1_3(rho1);
        let t185 = t184 * t184;
        let t187 = 1.0 / t185 / t169;
        let t188 = t183 * t187;
        let t192 = 1.0 / t185 / rho1;
        let t194 = sigma2 * t187;
        let t196 = tau1 * t192 - t194 / 8.0;
        let t197 = t196 * t48;
        let t198 = t197 * t53;
        let t200 = 5.0 / 9.0 * t198 - 1.0;
        let t201 = t53 * t200;
        let t204 = 1.0 + 0.2222222222222222 * t197 * t201;
        let t205 = rmath::sqrt(t204);
        let t206 = 1.0 / t205;
        let t209 = t80 * t194;
        let t210 = t209 / 36.0;
        let t211 = 9.0 / 20.0 * t200 * t206 + t210;
        let t212 = t211 * t211;
        let t215 = t169 * t169;
        let t216 = t215 * rho1;
        let t218 = 1.0 / t184 / t216;
        let t220 = t90 * t168 * t218;
        let t221 = 50.0 * t220;
        let t222 = 162.0 * t174 + t221;
        let t223 = rmath::sqrt(t222);
        let t226 = 3.291178445357254e-05 * t220;
        let t228 = t168 * sigma2;
        let t229 = t215 * t215;
        let t230 = 1.0 / t229;
        let t232 = 1.3522126526770064e-06 * t228 * t230;
        let t233 = t182 * t188 / 24.0 + 146.0 / 2025.0 * t212 - 73.0 / 97200.0 * t211 * t223 + t226 + 0.0020448759451792767 * t174 + t232;
        let t235 = 1.0 + 0.06134627835537829 * t209;
        let t236 = t235 * t235;
        let t237 = 1.0 / t236;
        let t239 = 0.804 + t233 * t237;
        let t241 = 0.646416 / t239;
        let t242 = -t200;
        let t243 = t242 * t242;
        let t244 = t243 * t242;
        let t245 = t196 * t196;
        let t246 = t245 * t87;
        let t247 = t246 * t89;
        let t249 = 1.0 + 0.6714891975308642 * t247;
        let t250 = rmath::sqrt(t249);
        let t252 = 1.0 / t250 / t249;
        let t253 = t244 * t252;
        let t255 = rmath::exp(-t209 / 8.0);
        let t257 = -0.45 + t210;
        let t258 = t257 * t257;
        let t260 = 10368.0 + t221;
        let t261 = rmath::sqrt(t260);
        let t264 = 0.029644443963477367 * t209 + 146.0 / 2025.0 * t258 - 73.0 / 97200.0 * t257 * t261 + t226 + 0.1308720604914737 + t232;
        let t266 = 0.804 + t264 * t237;
        let t269 = -0.646416 / t266 + t241;
        let t270 = t255 * t269;
        let t272 = 1.804 - t241 + t253 * t270;
        let t276 = piecewise3(t153, 0.0, -3.0 / 8.0 * t6 * t162 * t272);
        let tzk0 = t152 + t276;
        zk[ip] += tzk0;
    }
}
