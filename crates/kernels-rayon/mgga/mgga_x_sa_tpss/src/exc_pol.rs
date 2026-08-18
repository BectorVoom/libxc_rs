//! MGGA_X_SA_TPSS exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_sa_tpss.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_sa_tpss_exc_pol(
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
        let t29 = f64::sqrt(5.0);
        let t30 = M_PI * t29;
        let t31 = pow_1_3(rho0);
        let t32 = t31 * t31;
        let t34 = 1.0 / t32 / rho0;
        let t36 = rho0 * rho0;
        let t38 = 1.0 / t32 / t36;
        let t39 = sigma0 * t38;
        let t41 = tau0 * t34 - t39 / 8.0;
        let t42 = M_CBRT6;
        let t43 = t41 * t42;
        let t44 = M_PI * M_PI;
        let t45 = pow_1_3(t44);
        let t46 = t45 * t45;
        let t47 = 1.0 / t46;
        let t48 = t43 * t47;
        let t50 = 5.0 * t48 + 9.0;
        let t51 = f64::sqrt(t50);
        let t52 = 5.0 / 9.0 * t48;
        let t53 = t52 + 0.348;
        let t54 = f64::ln(t53);
        let t55 = 2.413 + t54;
        let t56 = f64::sqrt(t55);
        let t57 = 1.0 / t56;
        let t58 = t51 * t57;
        let t59 = t30 * t58;
        let t61 = sigma0 * sigma0;
        let t62 = 1.0 / t36;
        let t63 = t61 * t62;
        let t64 = tau0 * tau0;
        let t65 = 1.0 / t64;
        let t66 = t63 * t65;
        let t68 = 1.0 + t66 / 64.0;
        let t69 = t68 * t68;
        let t70 = 1.0 / t69;
        let t71 = t65 * t70;
        let t75 = (10.0 / 81.0 + 0.02485875 * t63 * t71) * t42;
        let t76 = t47 * sigma0;
        let t77 = t76 * t38;
        let t80 = t52 - 1.0;
        let t81 = t47 * t80;
        let t84 = 1.0 + 0.2222222222222222 * t43 * t81;
        let t85 = f64::sqrt(t84);
        let t86 = 1.0 / t85;
        let t89 = t42 * t47;
        let t90 = t89 * t39;
        let t92 = 9.0 / 20.0 * t80 * t86 + t90 / 36.0;
        let t93 = t92 * t92;
        let t96 = t42 * t42;
        let t98 = 1.0 / t45 / t44;
        let t99 = t96 * t98;
        let t100 = t36 * t36;
        let t101 = t100 * rho0;
        let t103 = 1.0 / t31 / t101;
        let t104 = t61 * t103;
        let t105 = t99 * t104;
        let t107 = 162.0 * t66 + 50.0 * t105;
        let t108 = f64::sqrt(t107);
        let t112 = 1.0 / M_PI * t29;
        let t113 = 1.0 / t51;
        let t115 = t112 * t113 * t56;
        let t119 = t61 * sigma0;
        let t120 = t100 * t100;
        let t121 = 1.0 / t120;
        let t124 = t75 * t77 / 24.0 + 146.0 / 2025.0 * t93 - 73.0 / 97200.0 * t92 * t108 + 25.0 / 209952.0 * t115 * t105 + 0.0017218861679299947 * t66 + 1.5033019185692233e-06 * t119 * t121;
        let t126 = 1.0 + 0.05165658503789984 * t90;
        let t127 = t126 * t126;
        let t128 = 1.0 / t127;
        let t130 = 2.0 / 45.0 * t59 + t124 * t128;
        let t131 = 1.0 / t130;
        let t135 = 1.0 - 2.0 / 45.0 * t30 * t58 * t131;
        let t139 = 1.0 + 2.0 / 45.0 * t30 * t58 * t135;
        let t143 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t139);
        let t144 = rho1 <= dens_threshold;
        let t145 = -t17;
        let t147 = piecewise5(t15, t12, t11, t16, t145 * t8);
        let t148 = 1.0 + t147;
        let t149 = t148 <= zeta_threshold;
        let t150 = pow_1_3(t148);
        let t152 = piecewise3(t149, t23, t150 * t148);
        let t153 = t152 * t27;
        let t154 = pow_1_3(rho1);
        let t155 = t154 * t154;
        let t157 = 1.0 / t155 / rho1;
        let t159 = rho1 * rho1;
        let t161 = 1.0 / t155 / t159;
        let t162 = sigma2 * t161;
        let t164 = tau1 * t157 - t162 / 8.0;
        let t165 = t164 * t42;
        let t166 = t165 * t47;
        let t168 = 5.0 * t166 + 9.0;
        let t169 = f64::sqrt(t168);
        let t170 = 5.0 / 9.0 * t166;
        let t171 = t170 + 0.348;
        let t172 = f64::ln(t171);
        let t173 = 2.413 + t172;
        let t174 = f64::sqrt(t173);
        let t175 = 1.0 / t174;
        let t176 = t169 * t175;
        let t177 = t30 * t176;
        let t179 = sigma2 * sigma2;
        let t180 = 1.0 / t159;
        let t181 = t179 * t180;
        let t182 = tau1 * tau1;
        let t183 = 1.0 / t182;
        let t184 = t181 * t183;
        let t186 = 1.0 + t184 / 64.0;
        let t187 = t186 * t186;
        let t188 = 1.0 / t187;
        let t189 = t183 * t188;
        let t193 = (10.0 / 81.0 + 0.02485875 * t181 * t189) * t42;
        let t194 = t47 * sigma2;
        let t195 = t194 * t161;
        let t198 = t170 - 1.0;
        let t199 = t47 * t198;
        let t202 = 1.0 + 0.2222222222222222 * t165 * t199;
        let t203 = f64::sqrt(t202);
        let t204 = 1.0 / t203;
        let t207 = t89 * t162;
        let t209 = 9.0 / 20.0 * t198 * t204 + t207 / 36.0;
        let t210 = t209 * t209;
        let t213 = t159 * t159;
        let t214 = t213 * rho1;
        let t216 = 1.0 / t154 / t214;
        let t217 = t179 * t216;
        let t218 = t99 * t217;
        let t220 = 162.0 * t184 + 50.0 * t218;
        let t221 = f64::sqrt(t220);
        let t224 = 1.0 / t169;
        let t226 = t112 * t224 * t174;
        let t230 = t179 * sigma2;
        let t231 = t213 * t213;
        let t232 = 1.0 / t231;
        let t235 = t193 * t195 / 24.0 + 146.0 / 2025.0 * t210 - 73.0 / 97200.0 * t209 * t221 + 25.0 / 209952.0 * t226 * t218 + 0.0017218861679299947 * t184 + 1.5033019185692233e-06 * t230 * t232;
        let t237 = 1.0 + 0.05165658503789984 * t207;
        let t238 = t237 * t237;
        let t239 = 1.0 / t238;
        let t241 = 2.0 / 45.0 * t177 + t235 * t239;
        let t242 = 1.0 / t241;
        let t246 = 1.0 - 2.0 / 45.0 * t30 * t176 * t242;
        let t250 = 1.0 + 2.0 / 45.0 * t30 * t176 * t246;
        let t254 = piecewise3(t144, 0.0, -3.0 / 8.0 * t6 * t153 * t250);
        let tzk0 = t143 + t254;
        zk[ip] += tzk0;
    }
}
