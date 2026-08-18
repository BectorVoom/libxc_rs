//! GGA_X_ITYH_OPTX vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh_optx.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ityh_optx_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_b: f64,
    param_a: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = M_CBRTPI;
        let t7 = 1.0 <= zeta_threshold;
        let t8 = zeta_threshold - 1.0;
        let t10 = piecewise5(t7, t8, t7, -t8, 0.0);
        let t11 = 1.0 + t10;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t11 <= zeta_threshold, t13 * zeta_threshold, t15 * t11);
        let t18 = t3 / t4 * t17;
        let t19 = pow_1_3(rho[ip]);
        let t20 = t3 * t3;
        let t22 = 1.0 / M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = M_CBRT4;
        let t26 = t24 * t25;
        let t27 = sigma[ip] * sigma[ip];
        let t28 = param_b * t27;
        let t29 = M_CBRT2;
        let t30 = rho[ip] * rho[ip];
        let t31 = t30 * t30;
        let t32 = t31 * rho[ip];
        let t34 = 1.0 / t19 / t32;
        let t36 = t29 * t29;
        let t38 = t19 * t19;
        let t40 = 1.0 / t38 / t30;
        let t43 = 1.0 + 6.0 * sigma[ip] * t36 * t40;
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t29 * t34 * t45;
        let t49 = param_a + 72.0 * t28 * t46;
        let t52 = M_PI * t20 * t26 / t49;
        let t53 = f64::sqrt(t52);
        let t55 = param_hyb_omega_0 / t53;
        let t56 = t11 * rho[ip];
        let t57 = pow_1_3(t56);
        let t58 = 1.0 / t57;
        let t59 = t29 * t58;
        let t61 = t55 * t59 / 2.0;
        let t62 = 1.35 <= t61;
        let t63 = 1.35 < t61;
        let t64 = piecewise3(t63, t61, 1.35);
        let t65 = t64 * t64;
        let t68 = t65 * t65;
        let t69 = 1.0 / t68;
        let t71 = t68 * t65;
        let t72 = 1.0 / t71;
        let t74 = t68 * t68;
        let t75 = 1.0 / t74;
        let t78 = 1.0 / t74 / t65;
        let t81 = 1.0 / t74 / t68;
        let t84 = 1.0 / t74 / t71;
        let t86 = t74 * t74;
        let t87 = 1.0 / t86;
        let t90 = piecewise3(t63, 1.35, t61);
        let t91 = f64::sqrt(M_PI);
        let t92 = 1.0 / t90;
        let t94 = erf_approx(t92 / 2.0);
        let t96 = t90 * t90;
        let t97 = 1.0 / t96;
        let t99 = f64::exp(-t97 / 4.0);
        let t100 = t99 - 1.0;
        let t103 = t99 - 3.0 / 2.0 - 2.0 * t96 * t100;
        let t106 = 2.0 * t90 * t103 + t91 * t94;
        let t110 = piecewise3(t62, 1.0 / t65 / 36.0 - t69 / 960.0 + t72 / 26880.0 - t75 / 829440.0 + t78 / 28385280.0 - t81 / 1073479680.0 + t84 / 44590694400.0 - t87 / 2021444812800.0, 1.0 - 8.0 / 3.0 * t90 * t106);
        let t111 = t19 * t110;
        let t115 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t111 * t49);
        let tzk0 = 2.0 * t115;
        zk[ip] += tzk0;
        let t116 = 1.0 / t38;
        let t117 = t116 * t110;
        let t121 = t65 * t64;
        let t122 = 1.0 / t121;
        let t125 = param_hyb_omega_0 / t53 / t52;
        let t127 = t125 * t59 * M_PI;
        let t128 = t20 * t24;
        let t129 = t49 * t49;
        let t130 = 1.0 / t129;
        let t131 = t25 * t130;
        let t132 = t31 * t30;
        let t134 = 1.0 / t19 / t132;
        let t136 = t29 * t134 * t45;
        let t140 = param_b * t27 * sigma[ip];
        let t141 = t31 * t31;
        let t142 = t141 * rho[ip];
        let t143 = 1.0 / t142;
        let t145 = 1.0 / t44 / t43;
        let t146 = t143 * t145;
        let t149 = -384.0 * t28 * t136 + 4608.0 * t140 * t146;
        let t155 = 1.0 / t57 / t56;
        let t156 = t29 * t155;
        let t160 = t127 * t128 * t131 * t149 / 4.0 - t55 * t156 * t11 / 6.0;
        let t161 = piecewise3(t63, t160, 0.0);
        let t164 = t68 * t64;
        let t165 = 1.0 / t164;
        let t168 = t68 * t121;
        let t169 = 1.0 / t168;
        let t173 = 1.0 / t74 / t64;
        let t177 = 1.0 / t74 / t121;
        let t181 = 1.0 / t74 / t164;
        let t185 = 1.0 / t74 / t168;
        let t189 = 1.0 / t86 / t64;
        let t193 = piecewise3(t63, 0.0, t160);
        let t195 = t99 * t97;
        let t199 = t96 * t90;
        let t200 = 1.0 / t199;
        let t204 = t90 * t100;
        let t209 = t200 * t193 * t99 / 2.0 - 4.0 * t204 * t193 - t92 * t193 * t99;
        let t212 = 2.0 * t193 * t103 - t195 * t193 + 2.0 * t90 * t209;
        let t216 = piecewise3(t62, -t122 * t161 / 18.0 + t165 * t161 / 240.0 - t169 * t161 / 4480.0 + t173 * t161 / 103680.0 - t177 * t161 / 2838528.0 + t181 * t161 / 89456640.0 - t185 * t161 / 3185049600.0 + t189 * t161 / 126340300800.0, -8.0 / 3.0 * t193 * t106 - 8.0 / 3.0 * t90 * t212);
        let t217 = t19 * t216;
        let t225 = piecewise3(t2, 0.0, -t18 * t117 * t49 / 8.0 - 3.0 / 8.0 * t18 * t217 * t49 - 3.0 / 8.0 * t18 * t111 * t149);
        let tvrho0 = 2.0 * rho[ip] * t225 + 2.0 * t115;
        vrho[ip] += tvrho0;
        let t228 = param_b * sigma[ip];
        let t231 = 1.0 / t141;
        let t232 = t231 * t145;
        let t235 = 144.0 * t228 * t46 - 1728.0 * t28 * t232;
        let t239 = t127 * t128 * t131 * t235 / 4.0;
        let t240 = piecewise3(t63, t239, 0.0);
        let t243 = t165 * t240;
        let t245 = t169 * t240;
        let t247 = t173 * t240;
        let t249 = t177 * t240;
        let t251 = t181 * t240;
        let t253 = t185 * t240;
        let t255 = t189 * t240;
        let t258 = piecewise3(t63, 0.0, t239);
        let t270 = t200 * t258 * t99 / 2.0 - 4.0 * t204 * t258 - t92 * t258 * t99;
        let t273 = 2.0 * t258 * t103 - t195 * t258 + 2.0 * t90 * t270;
        let t277 = piecewise3(t62, -t122 * t240 / 18.0 + t243 / 240.0 - t245 / 4480.0 + t247 / 103680.0 - t249 / 2838528.0 + t251 / 89456640.0 - t253 / 3185049600.0 + t255 / 126340300800.0, -8.0 / 3.0 * t258 * t106 - 8.0 / 3.0 * t90 * t273);
        let t278 = t19 * t277;
        let t285 = piecewise3(t2, 0.0, -3.0 / 8.0 * t18 * t111 * t235 - 3.0 / 8.0 * t18 * t278 * t49);
        let tvsigma0 = 2.0 * rho[ip] * t285;
        vsigma[ip] += tvsigma0;
    }
}
