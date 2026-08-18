//! MGGA_X_REVTM exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_revtm.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_revtm_exc_pol(
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
        let t33 = t30 * t31 / 8.0;
        let t34 = t33 < 1.0;
        let t35 = piecewise3(t34, t33, 1.0);
        let t36 = t35 * t35;
        let t37 = t36 * t35;
        let t39 = t36 + 3.0 * t37;
        let t40 = 1.0 + t37;
        let t41 = t40 * t40;
        let t42 = 1.0 / t41;
        let t43 = t39 * t42;
        let t44 = M_CBRT6;
        let t45 = M_PI * M_PI;
        let t46 = pow_1_3(t45);
        let t47 = t46 * t46;
        let t48 = 1.0 / t47;
        let t49 = t44 * t48;
        let t50 = rho0 * rho0;
        let t51 = pow_1_3(rho0);
        let t52 = t51 * t51;
        let t54 = 1.0 / t52 / t50;
        let t55 = sigma0 * t54;
        let t56 = t49 * t55;
        let t58 = t44 * t44;
        let t60 = 1.0 / t46 / t45;
        let t61 = t58 * t60;
        let t62 = sigma0 * sigma0;
        let t63 = t50 * t50;
        let t64 = t63 * rho0;
        let t66 = 1.0 / t51 / t64;
        let t70 = 1.0 + 0.1504548888888889 * t56 + 0.002689949046226295 * t61 * t62 * t66;
        let t71 = f64::powf(t70, 1.0 / 5.0);
        let t75 = 1.0 / t52 / rho0;
        let t76 = tau0 * t75;
        let t79 = 0.256337604 * t58 * t47;
        let t85 = 1.0 + 0.06394332777777778 * t56 - 5.0 / 9.0 * (0.14554132 * t76 + t79 + 0.011867481666666667 * t55) * t44 * t48;
        let t86 = t71 * t71;
        let t87 = 1.0 / t86;
        let t90 = 1.0 / t71 + 7.0 / 9.0 * t85 * t87;
        let t92 = 1.0 - t43;
        let t95 = (10.0 / 81.0 + 25.0 / 8748.0 * t56) * t44;
        let t96 = t48 * sigma0;
        let t101 = t76 - t55 / 8.0;
        let t102 = t101 * t44;
        let t105 = 5.0 / 9.0 * t102 * t48 - 1.0;
        let t106 = t48 * t105;
        let t109 = 1.0 + 0.2222222222222222 * t102 * t106;
        let t110 = f64::sqrt(t109);
        let t111 = 1.0 / t110;
        let t115 = 9.0 / 20.0 * t105 * t111 + t56 / 36.0;
        let t116 = t115 * t115;
        let t118 = t115 * t35;
        let t119 = 1.0 - t35;
        let t122 = 1.0 + 5.0 / 12.0 * t95 * t96 * t54 + 292.0 / 405.0 * t116 - 146.0 / 135.0 * t118 * t119;
        let t123 = f64::powf(t122, 1.0 / 10.0);
        let t125 = t92 * t123 + t43 * t90;
        let t129 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t125);
        let t130 = rho1 <= dens_threshold;
        let t131 = -t17;
        let t133 = piecewise5(t15, t12, t11, t16, t131 * t8);
        let t134 = 1.0 + t133;
        let t135 = t134 <= zeta_threshold;
        let t136 = pow_1_3(t134);
        let t138 = piecewise3(t135, t23, t136 * t134);
        let t139 = t138 * t27;
        let t140 = 1.0 / rho1;
        let t141 = sigma2 * t140;
        let t142 = 1.0 / tau1;
        let t144 = t141 * t142 / 8.0;
        let t145 = t144 < 1.0;
        let t146 = piecewise3(t145, t144, 1.0);
        let t147 = t146 * t146;
        let t148 = t147 * t146;
        let t150 = t147 + 3.0 * t148;
        let t151 = 1.0 + t148;
        let t152 = t151 * t151;
        let t153 = 1.0 / t152;
        let t154 = t150 * t153;
        let t155 = rho1 * rho1;
        let t156 = pow_1_3(rho1);
        let t157 = t156 * t156;
        let t159 = 1.0 / t157 / t155;
        let t160 = sigma2 * t159;
        let t161 = t49 * t160;
        let t163 = sigma2 * sigma2;
        let t164 = t155 * t155;
        let t165 = t164 * rho1;
        let t167 = 1.0 / t156 / t165;
        let t171 = 1.0 + 0.1504548888888889 * t161 + 0.002689949046226295 * t61 * t163 * t167;
        let t172 = f64::powf(t171, 1.0 / 5.0);
        let t176 = 1.0 / t157 / rho1;
        let t177 = tau1 * t176;
        let t184 = 1.0 + 0.06394332777777778 * t161 - 5.0 / 9.0 * (0.14554132 * t177 + t79 + 0.011867481666666667 * t160) * t44 * t48;
        let t185 = t172 * t172;
        let t186 = 1.0 / t185;
        let t189 = 1.0 / t172 + 7.0 / 9.0 * t184 * t186;
        let t191 = 1.0 - t154;
        let t194 = (10.0 / 81.0 + 25.0 / 8748.0 * t161) * t44;
        let t195 = t48 * sigma2;
        let t200 = t177 - t160 / 8.0;
        let t201 = t200 * t44;
        let t204 = 5.0 / 9.0 * t201 * t48 - 1.0;
        let t205 = t48 * t204;
        let t208 = 1.0 + 0.2222222222222222 * t201 * t205;
        let t209 = f64::sqrt(t208);
        let t210 = 1.0 / t209;
        let t214 = 9.0 / 20.0 * t204 * t210 + t161 / 36.0;
        let t215 = t214 * t214;
        let t217 = t214 * t146;
        let t218 = 1.0 - t146;
        let t221 = 1.0 + 5.0 / 12.0 * t194 * t195 * t159 + 292.0 / 405.0 * t215 - 146.0 / 135.0 * t217 * t218;
        let t222 = f64::powf(t221, 1.0 / 10.0);
        let t224 = t154 * t189 + t191 * t222;
        let t228 = piecewise3(t130, 0.0, -3.0 / 8.0 * t6 * t139 * t224);
        let tzk0 = t129 + t228;
        zk[ip] += tzk0;
    }
}
