//! MGGA_X_RSCAN exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rscan.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rscan_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_alphar: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    param_taur: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = M_CBRT3;
        let t5 = M_CBRTPI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t16 = pow_1_3(t12);
        let t18 = piecewise3(t12 <= zeta_threshold, t14 * zeta_threshold, t16 * t12);
        let t19 = t4 / t5 * t18;
        let t20 = pow_1_3(rho[ip]);
        let t21 = M_CBRT6;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = t23 * t23;
        let t25 = 1.0 / t24;
        let t26 = t21 * t25;
        let t27 = M_CBRT2;
        let t28 = t27 * t27;
        let t29 = sigma[ip] * t28;
        let t30 = rho[ip] * rho[ip];
        let t31 = t20 * t20;
        let t33 = 1.0 / t31 / t30;
        let t34 = t29 * t33;
        let t35 = t26 * t34;
        let t39 = 100.0 / 6561.0 / param_k1 - 73.0 / 648.0;
        let t40 = t21 * t21;
        let t42 = t23 * t22;
        let t43 = 1.0 / t42;
        let t44 = t39 * t40 * t43;
        let t45 = sigma[ip] * sigma[ip];
        let t46 = t45 * t27;
        let t47 = t30 * t30;
        let t48 = t47 * rho[ip];
        let t50 = 1.0 / t20 / t48;
        let t55 = rmath::exp(-27.0 / 80.0 * t39 * t21 * t25 * t34);
        let t56 = t50 * t55;
        let t60 = rmath::sqrt(146.0);
        let t61 = t60 * t21;
        let t62 = t61 * t25;
        let t65 = t12 * t12;
        let t66 = t65 * t65;
        let t67 = t66 * t12;
        let t68 = t67 * t48;
        let t69 = tau[ip] * t28;
        let t70 = t31 * rho[ip];
        let t71 = 1.0 / t70;
        let t74 = t69 * t71 - t34 / 8.0;
        let t75 = 0.0 < t74;
        let t76 = piecewise3(t75, t74, 0.0);
        let t77 = t76 * t76;
        let t78 = t77 * t76;
        let t79 = t12 * rho[ip];
        let t80 = pow_1_3(t79);
        let t81 = t80 * t80;
        let t84 = t40 * t24;
        let t88 = 3.0 / 40.0 * t27 * t81 * t79 * t84 + param_taur / 2.0;
        let t89 = t88 * t88;
        let t90 = t89 * t88;
        let t91 = 1.0 / t90;
        let t93 = t65 * t12;
        let t94 = t30 * rho[ip];
        let t96 = t80 * t93 * t94;
        let t97 = t28 * t96;
        let t98 = 1.0 / t89;
        let t99 = t77 * t98;
        let t102 = t97 * t99 / 16.0 + param_alphar;
        let t103 = 1.0 / t102;
        let t104 = t78 * t91 * t103;
        let t106 = t68 * t104 / 32.0;
        let t107 = 1.0 - t106;
        let t109 = t107 * t107;
        let t111 = rmath::exp(-t109 / 2.0);
        let t114 = 7.0 / 12960.0 * t62 * t34 + t60 * t107 * t111 / 100.0;
        let t115 = t114 * t114;
        let t116 = param_k1 + 5.0 / 972.0 * t35 + t44 * t46 * t56 / 288.0 + t115;
        let t121 = 1.0 + param_k1 * (1.0 - param_k1 / t116);
        let t122 = t106 <= 2.5;
        let t123 = 2.5 < t106;
        let t124 = piecewise3(t123, 2.5, t106);
        let t126 = t124 * t124;
        let t128 = t126 * t124;
        let t130 = t126 * t126;
        let t132 = t130 * t124;
        let t134 = t130 * t126;
        let t139 = piecewise3(t123, t106, 2.5);
        let t140 = 1.0 - t139;
        let t143 = rmath::exp(param_c2 / t140);
        let t145 = piecewise3(t122, 1.0 - 0.667 * t124 - 0.4445555 * t126 - 0.663086601049 * t128 + 1.45129704449 * t130 - 0.887998041597 * t132 + 0.234528941479 * t134 - 0.023185843322 * t130 * t128, -param_d * t143);
        let t146 = 1.0 - t145;
        let t149 = t121 * t146 + 1.174 * t145;
        let t151 = rmath::sqrt(3.0);
        let t152 = 1.0 / t23;
        let t153 = t40 * t152;
        let t154 = rmath::sqrt(sigma[ip]);
        let t155 = t154 * t27;
        let t157 = 1.0 / t20 / rho[ip];
        let t159 = t153 * t155 * t157;
        let t160 = rmath::sqrt(t159);
        let t164 = rmath::exp(-9.8958 * t151 / t160);
        let t165 = 1.0 - t164;
        let t169 = piecewise3(t3, 0.0, -3.0 / 8.0 * t19 * t20 * t149 * t165);
        let tzk0 = 2.0 * t169;
        zk[ip] += tzk0;
    }
}
