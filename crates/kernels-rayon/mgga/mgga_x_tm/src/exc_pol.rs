//! MGGA_X_TM exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_tm.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_tm_exc_pol(
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
        let t70 = 1.0 + 0.15045488888888888889e0 * t56 + 0.26899490462262948e-2 * t61 * t62 * t66;
        let t71 = f64::powf(t70, 1.0 / 5.0);
        let t75 = 1.0 / t52 / rho0;
        let t76 = tau0 * t75;
        let t79 = 0.256337604e0 * t58 * t47;
        let t85 = 1.0 + 0.63943327777777777778e-1 * t56 - 5.0 / 9.0 * (0.14554132e0 * t76 + t79 + 0.11867481666666666667e-1 * t55) * t44 * t48;
        let t86 = t71 * t71;
        let t87 = 1.0 / t86;
        let t90 = 1.0 / t71 + 7.0 / 9.0 * t85 * t87;
        let t92 = 1.0 - t43;
        let t95 = (10.0 / 81.0 + 25.0 / 8748.0 * t56) * t44;
        let t96 = t48 * sigma0;
        let t106 = (t76 - t55 / 8.0) * t44 * t48 / 4.0 - 9.0 / 20.0 + t56 / 36.0;
        let t107 = t106 * t106;
        let t109 = t106 * t35;
        let t110 = 1.0 - t35;
        let t113 = 1.0 + 5.0 / 12.0 * t95 * t96 * t54 + 292.0 / 405.0 * t107 - 146.0 / 135.0 * t109 * t110;
        let t114 = f64::powf(t113, 1.0 / 10.0);
        let t116 = t92 * t114 + t43 * t90;
        let t120 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t28 * t116);
        let t121 = rho1 <= dens_threshold;
        let t122 = -t17;
        let t124 = piecewise5(t15, t12, t11, t16, t122 * t8);
        let t125 = 1.0 + t124;
        let t126 = t125 <= zeta_threshold;
        let t127 = pow_1_3(t125);
        let t129 = piecewise3(t126, t23, t127 * t125);
        let t130 = t129 * t27;
        let t131 = 1.0 / rho1;
        let t132 = sigma2 * t131;
        let t133 = 1.0 / tau1;
        let t135 = t132 * t133 / 8.0;
        let t136 = t135 < 1.0;
        let t137 = piecewise3(t136, t135, 1.0);
        let t138 = t137 * t137;
        let t139 = t138 * t137;
        let t141 = t138 + 3.0 * t139;
        let t142 = 1.0 + t139;
        let t143 = t142 * t142;
        let t144 = 1.0 / t143;
        let t145 = t141 * t144;
        let t146 = rho1 * rho1;
        let t147 = pow_1_3(rho1);
        let t148 = t147 * t147;
        let t150 = 1.0 / t148 / t146;
        let t151 = sigma2 * t150;
        let t152 = t49 * t151;
        let t154 = sigma2 * sigma2;
        let t155 = t146 * t146;
        let t156 = t155 * rho1;
        let t158 = 1.0 / t147 / t156;
        let t162 = 1.0 + 0.15045488888888888889e0 * t152 + 0.26899490462262948e-2 * t61 * t154 * t158;
        let t163 = f64::powf(t162, 1.0 / 5.0);
        let t167 = 1.0 / t148 / rho1;
        let t168 = tau1 * t167;
        let t175 = 1.0 + 0.63943327777777777778e-1 * t152 - 5.0 / 9.0 * (0.14554132e0 * t168 + t79 + 0.11867481666666666667e-1 * t151) * t44 * t48;
        let t176 = t163 * t163;
        let t177 = 1.0 / t176;
        let t180 = 1.0 / t163 + 7.0 / 9.0 * t175 * t177;
        let t182 = 1.0 - t145;
        let t185 = (10.0 / 81.0 + 25.0 / 8748.0 * t152) * t44;
        let t186 = t48 * sigma2;
        let t196 = (t168 - t151 / 8.0) * t44 * t48 / 4.0 - 9.0 / 20.0 + t152 / 36.0;
        let t197 = t196 * t196;
        let t199 = t196 * t137;
        let t200 = 1.0 - t137;
        let t203 = 1.0 + 5.0 / 12.0 * t185 * t186 * t150 + 292.0 / 405.0 * t197 - 146.0 / 135.0 * t199 * t200;
        let t204 = f64::powf(t203, 1.0 / 10.0);
        let t206 = t145 * t180 + t182 * t204;
        let t210 = piecewise3(t121, 0.0, -3.0 / 8.0 * t6 * t130 * t206);
        let tzk0 = t120 + t210;
        zk[ip] += tzk0;
    }
}
