//! GGA_X_WC fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_wc.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_wc_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t6 = t3 / t4;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t20 = M_CBRT6;
        let t21 = M_PI * M_PI;
        let t22 = pow_1_3(t21);
        let t23 = t22 * t22;
        let t24 = 1.0 / t23;
        let t25 = t20 * t24;
        let t26 = M_CBRT2;
        let t27 = t26 * t26;
        let t28 = sigma[ip] * t27;
        let t29 = rho[ip] * rho[ip];
        let t30 = t18 * t18;
        let t32 = 1.0 / t30 / t29;
        let t34 = t25 * t28 * t32;
        let t36 = t25 * sigma[ip];
        let t37 = t27 * t32;
        let t39 = rmath::exp(-t34 / 24.0);
        let t40 = t37 * t39;
        let t43 = t20 * t20;
        let t46 = t43 / t22 / t21;
        let t47 = sigma[ip] * sigma[ip];
        let t49 = t29 * t29;
        let t50 = t49 * rho[ip];
        let t52 = 1.0 / t18 / t50;
        let t56 = 1.0 + 2.7560657413756314e-05 * t46 * t47 * t26 * t52;
        let t57 = rmath::ln(t56);
        let t58 = 0.804 + 5.0 / 972.0 * t34 + 0.004002424276710846 * t36 * t40 + t57;
        let t61 = 1.804 - 0.646416 / t58;
        let t65 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t17 * t18 * t61);
        let tzk0 = 2.0 * t65;
        zk[ip] += tzk0;
        let t66 = 1.0 / t30;
        let t71 = t3 * t17;
        let t72 = t58 * t58;
        let t73 = 1.0 / t72;
        let t74 = t18 * t73;
        let t75 = t29 * rho[ip];
        let t77 = 1.0 / t30 / t75;
        let t81 = t27 * t77;
        let t82 = t81 * t39;
        let t85 = t46 * t47;
        let t86 = t49 * t29;
        let t88 = 1.0 / t18 / t86;
        let t89 = t26 * t88;
        let t90 = t89 * t39;
        let t93 = 1.0 / t56;
        let t94 = t89 * t93;
        let t97 = -10.0 / 729.0 * t25 * t28 * t77 - 0.010673131404562256 * t36 * t82 + 0.0008894276170468547 * t85 * t90 - 0.00014699017287336702 * t85 * t94;
        let t102 = piecewise3(t2, 0.0, -t6 * t17 * t66 * t61 / 8.0 - 0.1655109536374632 * t71 * t74 * t97);
        let tvrho0 = 2.0 * rho[ip] * t102 + 2.0 * t65;
        vrho[ip] += tvrho0;
        let t109 = t46 * sigma[ip];
        let t110 = t26 * t52;
        let t111 = t110 * t39;
        let t114 = t110 * t93;
        let t117 = 5.0 / 972.0 * t25 * t37 + 0.004002424276710846 * t25 * t40 - 0.0003335353563925705 * t109 * t111 + 5.512131482751263e-05 * t109 * t114;
        let t121 = piecewise3(t2, 0.0, -0.1655109536374632 * t71 * t74 * t117);
        let tvsigma0 = 2.0 * rho[ip] * t121;
        vsigma[ip] += tvsigma0;
        let t125 = 1.0 / t30 / rho[ip];
        let t130 = t66 * t73;
        let t135 = 1.0 / t72 / t58;
        let t136 = t18 * t135;
        let t137 = t97 * t97;
        let t142 = 1.0 / t30 / t49;
        let t146 = t27 * t142;
        let t147 = t146 * t39;
        let t150 = t49 * t75;
        let t152 = 1.0 / t18 / t150;
        let t153 = t26 * t152;
        let t154 = t153 * t39;
        let t157 = t21 * t21;
        let t158 = 1.0 / t157;
        let t159 = t47 * sigma[ip];
        let t160 = t158 * t159;
        let t161 = t49 * t49;
        let t162 = t161 * t29;
        let t163 = 1.0 / t162;
        let t167 = t153 * t93;
        let t172 = t20 / t23 / t157;
        let t173 = t47 * t47;
        let t174 = t172 * t173;
        let t175 = t161 * t49;
        let t177 = 1.0 / t30 / t175;
        let t179 = t56 * t56;
        let t180 = 1.0 / t179;
        let t181 = t27 * t177 * t180;
        let t184 = 110.0 / 2187.0 * t25 * t28 * t142 + 0.039134815150061605 * t36 * t147 - 0.008004848553421692 * t85 * t154 + 0.0011859034893958063 * t160 * t163 * t39 + 0.0009309377615313244 * t85 * t167 - 1.2963666552805393e-07 * t174 * t181;
        let t189 = piecewise3(t2, 0.0, t6 * t17 * t125 * t61 / 12.0 - 0.1103406357583088 * t71 * t130 * t97 + 0.3310219072749264 * t71 * t136 * t137 - 0.1655109536374632 * t71 * t74 * t184);
        let tv2rho20 = 2.0 * rho[ip] * t189 + 4.0 * t102;
        v2rho2[ip] += tv2rho20;
        let t195 = t71 * t18;
        let t196 = t135 * t117;
        let t197 = t196 * t97;
        let t204 = t46 * t26;
        let t209 = t158 * t47;
        let t210 = t161 * rho[ip];
        let t211 = 1.0 / t210;
        let t217 = t172 * t159;
        let t218 = t161 * t75;
        let t220 = 1.0 / t30 / t218;
        let t225 = -10.0 / 729.0 * t25 * t81 - 0.010673131404562256 * t25 * t82 + 0.002668282851140564 * t204 * t88 * sigma[ip] * t39 - 0.00044471380852342736 * t209 * t211 * t39 - 0.00029398034574673403 * t109 * t94 + 4.861374957302022e-08 * t217 * t27 * t220 * t180;
        let t230 = piecewise3(t2, 0.0, -0.0551703178791544 * t71 * t130 * t117 + 0.3310219072749264 * t195 * t197 - 0.1655109536374632 * t71 * t74 * t225);
        let tv2rhosigma0 = 2.0 * rho[ip] * t230 + 2.0 * t121;
        v2rhosigma[ip] += tv2rhosigma0;
        let t233 = t117 * t117;
        let t239 = t158 * sigma[ip];
        let t240 = 1.0 / t161;
        let t248 = 1.0 / t30 / t162;
        let t250 = t27 * t248 * t180;
        let t253 = -0.000667070712785141 * t46 * t111 + 0.00016676767819628525 * t239 * t240 * t39 + 5.512131482751263e-05 * t46 * t114 - 1.8230156089882582e-08 * t172 * t47 * t250;
        let t258 = piecewise3(t2, 0.0, 0.3310219072749264 * t71 * t136 * t233 - 0.1655109536374632 * t71 * t74 * t253);
        let tv2sigma20 = 2.0 * rho[ip] * t258;
        v2sigma2[ip] += tv2sigma20;
    }
}
