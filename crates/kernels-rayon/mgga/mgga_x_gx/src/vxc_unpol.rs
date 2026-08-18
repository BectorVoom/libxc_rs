//! MGGA_X_GX vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_gx.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{Heaviside, piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_gx_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c0: f64,
    param_c1: f64,
    param_alphainf: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t7 = t4 / t5;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = pow_1_3(rho[ip]);
        let t20 = t18 * t19;
        let t21 = M_CBRT2;
        let t22 = t4 * t4;
        let t24 = M_CBRT4;
        let t26 = 8.0 / 27.0 * t21 * t22 * t24;
        let t27 = t21 * t21;
        let t28 = tau[ip] * t27;
        let t29 = t19 * t19;
        let t31 = 1.0 / t29 / rho[ip];
        let t33 = sigma[ip] * t27;
        let t34 = rho[ip] * rho[ip];
        let t36 = 1.0 / t29 / t34;
        let t39 = t28 * t31 - t33 * t36 / 8.0;
        let t40 = M_CBRT6;
        let t42 = M_PI * M_PI;
        let t43 = pow_1_3(t42);
        let t44 = t43 * t43;
        let t45 = 1.0 / t44;
        let t46 = t39 * t40 * t45;
        let t48 = t40 * t45;
        let t51 = param_c0 + 5.0 / 9.0 * param_c1 * t39 * t48;
        let t52 = param_c0 + param_c1 - 1.0;
        let t56 = 1.0 + 5.0 / 9.0 * t52 * t39 * t48;
        let t57 = 1.0 / t56;
        let t59 = 1.0 - t26;
        let t60 = t51 * t57 * t59;
        let t63 = t26 + 5.0 / 9.0 * t46 * t60;
        let t64 = 5.0 / 9.0 * t46;
        let t65 = 1.0 - t64;
        let t66 = Heaviside(t65);
        let t68 = 1.0 - param_alphainf;
        let t69 = t68 * t65;
        let t70 = 1.0 + t64;
        let t71 = 1.0 / t70;
        let t73 = t69 * t71 + 1.0;
        let t74 = -t65;
        let t75 = Heaviside(t74);
        let t77 = t63 * t66 + t73 * t75;
        let t81 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t77);
        let tzk0 = 2.0 * t81;
        zk[ip] += tzk0;
        let t83 = t18 / t29;
        let t89 = t34 * rho[ip];
        let t91 = 1.0 / t29 / t89;
        let t94 = -5.0 / 3.0 * t28 * t36 + t33 * t91 / 3.0;
        let t96 = t94 * t40 * t45;
        let t99 = t40 * t40;
        let t100 = t39 * t99;
        let t102 = 1.0 / t43 / t42;
        let t103 = t100 * t102;
        let t105 = t57 * t59;
        let t106 = param_c1 * t94 * t105;
        let t109 = t102 * t51;
        let t110 = t100 * t109;
        let t111 = t56 * t56;
        let t112 = 1.0 / t111;
        let t113 = t112 * t59;
        let t115 = t113 * t52 * t94;
        let t118 = 5.0 / 9.0 * t96 * t60 + 25.0 / 81.0 * t103 * t106 - 25.0 / 81.0 * t110 * t115;
        let t120 = 0.0;
        let t121 = t63 * t120;
        let t125 = t48 * t71;
        let t127 = t70 * t70;
        let t128 = 1.0 / t127;
        let t129 = t69 * t128;
        let t132 = -5.0 / 9.0 * t68 * t94 * t125 - 5.0 / 9.0 * t129 * t96;
        let t134 = t73 * t120;
        let t137 = t118 * t66 - 5.0 / 9.0 * t121 * t96 + t132 * t75 + 5.0 / 9.0 * t134 * t96;
        let t142 = piecewise3(t3, 0.0, -t7 * t83 * t77 / 8.0 - 3.0 / 8.0 * t7 * t20 * t137);
        let tvrho0 = 2.0 * rho[ip] * t142 + 2.0 * t81;
        vrho[ip] += tvrho0;
        let t145 = t27 * t36;
        let t148 = t45 * t51 * t105;
        let t149 = t145 * t40 * t148;
        let t151 = t102 * param_c1;
        let t152 = t100 * t151;
        let t154 = t152 * t145 * t105;
        let t156 = t52 * t27;
        let t159 = t110 * t113 * t156 * t36;
        let t161 = -5.0 / 72.0 * t149 - 25.0 / 648.0 * t154 + 25.0 / 648.0 * t159;
        let t163 = t121 * t27;
        let t165 = t36 * t40 * t45;
        let t166 = t163 * t165;
        let t168 = t68 * t27;
        let t169 = t168 * t36;
        let t170 = t169 * t125;
        let t172 = t129 * t145 * t48;
        let t174 = 5.0 / 72.0 * t170 + 5.0 / 72.0 * t172;
        let t176 = t134 * t27;
        let t177 = t176 * t165;
        let t179 = t161 * t66 + 5.0 / 72.0 * t166 + t174 * t75 - 5.0 / 72.0 * t177;
        let t183 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t179);
        let tvsigma0 = 2.0 * rho[ip] * t183;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t185 = t27 * t31;
        let t196 = 5.0 / 9.0 * t185 * t40 * t148 + 25.0 / 81.0 * t152 * t185 * t105 - 25.0 / 81.0 * t110 * t113 * t156 * t31;
        let t199 = t31 * t40 * t45;
        let t202 = t168 * t31;
        let t207 = -5.0 / 9.0 * t129 * t185 * t48 - 5.0 / 9.0 * t202 * t125;
        let t211 = t196 * t66 - 5.0 / 9.0 * t163 * t199 + t207 * t75 + 5.0 / 9.0 * t176 * t199;
        let t215 = piecewise3(t3, 0.0, -3.0 / 8.0 * t7 * t20 * t211);
        let tvtau0 = 2.0 * rho[ip] * t215;
        vtau[ip] += tvtau0;
    }
}
