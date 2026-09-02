//! GGA_X_ITYH_OPTX exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_optx.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ityh_optx_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_b: f64,
    param_a: f64,
    param_hyb_omega_0: f64,
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
        let t26 = t5 * t25;
        let t27 = pow_1_3(t6);
        let t28 = t2 * t2;
        let t29 = M_PI * t28;
        let t30 = 1.0 / M_PI;
        let t31 = pow_1_3(t30);
        let t32 = 1.0 / t31;
        let t33 = M_CBRT4;
        let t34 = t32 * t33;
        let t35 = sigma0 * sigma0;
        let t36 = param_b * t35;
        let t37 = rho0 * rho0;
        let t38 = t37 * t37;
        let t39 = t38 * rho0;
        let t40 = pow_1_3(rho0);
        let t42 = 1.0 / t40 / t39;
        let t43 = t40 * t40;
        let t48 = 1.0 + 6.0 * sigma0 / t43 / t37;
        let t49 = t48 * t48;
        let t50 = 1.0 / t49;
        let t51 = t42 * t50;
        let t54 = param_a + 36.0 * t36 * t51;
        let t57 = t29 * t34 / t54;
        let t58 = rmath::sqrt(t57);
        let t60 = param_hyb_omega_0 / t58;
        let t61 = M_CBRT2;
        let t62 = t19 * t6;
        let t63 = pow_1_3(t62);
        let t64 = 1.0 / t63;
        let t65 = t61 * t64;
        let t67 = t60 * t65 / 2.0;
        let t68 = 1.35 <= t67;
        let t69 = 1.35 < t67;
        let t70 = piecewise3(t69, t67, 1.35);
        let t71 = t70 * t70;
        let t74 = t71 * t71;
        let t75 = 1.0 / t74;
        let t77 = t74 * t71;
        let t78 = 1.0 / t77;
        let t80 = t74 * t74;
        let t81 = 1.0 / t80;
        let t84 = 1.0 / t80 / t71;
        let t87 = 1.0 / t80 / t74;
        let t90 = 1.0 / t80 / t77;
        let t92 = t80 * t80;
        let t93 = 1.0 / t92;
        let t96 = piecewise3(t69, 1.35, t67);
        let t97 = rmath::sqrt(M_PI);
        let t98 = 1.0 / t96;
        let t100 = rmath::erf(t98 / 2.0);
        let t102 = t96 * t96;
        let t103 = 1.0 / t102;
        let t105 = rmath::exp(-t103 / 4.0);
        let t106 = t105 - 1.0;
        let t109 = t105 - 3.0 / 2.0 - 2.0 * t102 * t106;
        let t112 = t97 * t100 + 2.0 * t96 * t109;
        let t116 = piecewise3(t68, 1.0 / t71 / 36.0 - t75 / 960.0 + t78 / 26880.0 - t81 / 829440.0 + t84 / 28385280.0 - t87 / 1073479680.0 + t90 / 44590694400.0 - t93 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t96 * t112);
        let t117 = t27 * t116;
        let t118 = t117 * t54;
        let t121 = piecewise3(t1, 0.0, -3.0 / 8.0 * t26 * t118);
        let t122 = rho1 <= dens_threshold;
        let t123 = -t16;
        let t125 = piecewise5(t14, t11, t10, t15, t123 * t7);
        let t126 = 1.0 + t125;
        let t127 = t126 <= zeta_threshold;
        let t128 = pow_1_3(t126);
        let t130 = piecewise3(t127, t22, t128 * t126);
        let t131 = t5 * t130;
        let t132 = sigma2 * sigma2;
        let t133 = param_b * t132;
        let t134 = rho1 * rho1;
        let t135 = t134 * t134;
        let t136 = t135 * rho1;
        let t137 = pow_1_3(rho1);
        let t139 = 1.0 / t137 / t136;
        let t140 = t137 * t137;
        let t145 = 1.0 + 6.0 * sigma2 / t140 / t134;
        let t146 = t145 * t145;
        let t147 = 1.0 / t146;
        let t148 = t139 * t147;
        let t151 = param_a + 36.0 * t133 * t148;
        let t154 = t29 * t34 / t151;
        let t155 = rmath::sqrt(t154);
        let t157 = param_hyb_omega_0 / t155;
        let t158 = t126 * t6;
        let t159 = pow_1_3(t158);
        let t160 = 1.0 / t159;
        let t161 = t61 * t160;
        let t163 = t157 * t161 / 2.0;
        let t164 = 1.35 <= t163;
        let t165 = 1.35 < t163;
        let t166 = piecewise3(t165, t163, 1.35);
        let t167 = t166 * t166;
        let t170 = t167 * t167;
        let t171 = 1.0 / t170;
        let t173 = t170 * t167;
        let t174 = 1.0 / t173;
        let t176 = t170 * t170;
        let t177 = 1.0 / t176;
        let t180 = 1.0 / t176 / t167;
        let t183 = 1.0 / t176 / t170;
        let t186 = 1.0 / t176 / t173;
        let t188 = t176 * t176;
        let t189 = 1.0 / t188;
        let t192 = piecewise3(t165, 1.35, t163);
        let t193 = 1.0 / t192;
        let t195 = rmath::erf(t193 / 2.0);
        let t197 = t192 * t192;
        let t198 = 1.0 / t197;
        let t200 = rmath::exp(-t198 / 4.0);
        let t201 = t200 - 1.0;
        let t204 = t200 - 3.0 / 2.0 - 2.0 * t197 * t201;
        let t207 = 2.0 * t192 * t204 + t97 * t195;
        let t211 = piecewise3(t164, 1.0 / t167 / 36.0 - t171 / 960.0 + t174 / 26880.0 - t177 / 829440.0 + t180 / 28385280.0 - t183 / 1073479680.0 + t186 / 44590694400.0 - t189 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t192 * t207);
        let t212 = t27 * t211;
        let t213 = t212 * t151;
        let t216 = piecewise3(t122, 0.0, -3.0 / 8.0 * t131 * t213);
        let tzk0 = t121 + t216;
        zk[ip] += tzk0;
    }
}
