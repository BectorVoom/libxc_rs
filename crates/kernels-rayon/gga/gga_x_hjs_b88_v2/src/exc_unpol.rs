//! GGA_X_HJS_B88_V2 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hjs_b88_v2.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_hjs_b88_v2_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_hyb_omega_0: f64,
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
        let t12 = t11 <= zeta_threshold;
        let t13 = pow_1_3(zeta_threshold);
        let t15 = pow_1_3(t11);
        let t17 = piecewise3(t12, t13 * zeta_threshold, t15 * t11);
        let t18 = pow_1_3(rho[ip]);
        let t19 = t17 * t18;
        let t20 = t3 * t3;
        let t21 = param_hyb_omega_0 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = piecewise3(t12, t13, t15);
        let t27 = 1.0 / t26;
        let t28 = 1.0 / t18;
        let t29 = t27 * t28;
        let t30 = M_CBRT6;
        let t31 = t30 * t30;
        let t32 = t31 * t24;
        let t33 = rmath::sqrt(sigma[ip]);
        let t34 = M_CBRT2;
        let t35 = t33 * t34;
        let t37 = 1.0 / t18 / rho[ip];
        let t41 = rmath::exp(-t32 * t35 * t37 / 12.0);
        let t42 = rmath::exp(20.0);
        let t44 = 1.0 / (t42 - 1.0);
        let t45 = t41 + t44;
        let t49 = rmath::ln(t45 / (1.0 + t44));
        let t50 = t49 * t49;
        let t51 = param_a_0;
        let t53 = param_a_1;
        let t54 = t50 * t49;
        let t56 = param_a_2;
        let t57 = t50 * t50;
        let t59 = param_a_3;
        let t60 = t57 * t49;
        let t62 = param_a_4;
        let t63 = t57 * t50;
        let t65 = param_a_5;
        let t66 = t57 * t54;
        let t68 = t50 * t51 - t53 * t54 + t56 * t57 - t59 * t60 + t62 * t63 - t65 * t66;
        let t69 = t50 * t68;
        let t70 = param_b_0;
        let t72 = param_b_1;
        let t74 = param_b_2;
        let t76 = param_b_3;
        let t78 = param_b_4;
        let t80 = param_b_5;
        let t82 = param_b_6;
        let t84 = param_b_7;
        let t85 = t57 * t57;
        let t87 = param_b_8;
        let t90 = -t49 * t85 * t87 - t49 * t70 + t50 * t72 - t54 * t74 + t57 * t76 - t60 * t78 + t63 * t80 - t66 * t82 + t84 * t85 + 1.0;
        let t91 = 1.0 / t90;
        let t92 = t69 * t91;
        let t93 = 1e-10 < t92;
        let t94 = piecewise3(t93, t92, 1e-10);
        let t95 = param_hyb_omega_0 * param_hyb_omega_0;
        let t96 = t95 * t3;
        let t97 = t23 * t23;
        let t98 = 1.0 / t97;
        let t99 = t26 * t26;
        let t101 = t98 / t99;
        let t102 = t18 * t18;
        let t103 = 1.0 / t102;
        let t105 = t96 * t101 * t103;
        let t107 = 0.60965 + t94 + t105 / 3.0;
        let t108 = rmath::sqrt(t107);
        let t109 = 1.0 / t108;
        let t111 = t25 * t29 * t109;
        let t113 = 1.0 - t111 / 3.0;
        let t114 = 0.60965 + t94;
        let t115 = 1.0 / t114;
        let t119 = 1.0 + t50 / 4.0;
        let t120 = 1.0 / t119;
        let t124 = 1.0 + 0.3121563353845126 * t50 * t120 + 4.21411052769092 * t94;
        let t126 = 1.0 / t22;
        let t127 = t95 * param_hyb_omega_0 * t126;
        let t129 = 1.0 / t99 / t26;
        let t130 = 1.0 / rho[ip];
        let t131 = t129 * t130;
        let t133 = 1.0 / t108 / t107;
        let t135 = t127 * t131 * t133;
        let t137 = 2.0 - t111 + t135 / 3.0;
        let t138 = t124 * t137;
        let t139 = t114 * t114;
        let t140 = 1.0 / t139;
        let t146 = t139 * t114;
        let t148 = rmath::sqrt(t114);
        let t149 = t148 * t146;
        let t150 = rmath::sqrt(M_PI);
        let t152 = rmath::sqrt(t94);
        let t155 = 0.0 < 0.7572109999 + t94;
        let t157 = piecewise3(t155, 0.757211 + t94, 1e-10);
        let t158 = rmath::sqrt(t157);
        let t160 = 4.0 / 5.0 * t150 + 12.0 / 5.0 * t152 - 12.0 / 5.0 * t158;
        let t162 = 0.0474596 * t124 * t114 + 0.028363733333333332 * t139 - 0.9086532 * t146 - t149 * t160;
        let t165 = t95 * t95;
        let t167 = t165 * param_hyb_omega_0 * t3;
        let t169 = 1.0 / t97 / t22;
        let t170 = t167 * t169;
        let t171 = t99 * t99;
        let t173 = 1.0 / t171 / t26;
        let t175 = 1.0 / t102 / rho[ip];
        let t176 = t173 * t175;
        let t177 = t107 * t107;
        let t179 = 1.0 / t108 / t177;
        let t183 = 8.0 - 5.0 * t111 + 10.0 / 3.0 * t135 - t170 * t176 * t179 / 3.0;
        let t184 = t162 * t183;
        let t185 = 1.0 / t146;
        let t189 = 3.0 * t105;
        let t190 = 9.0 * t94 + t189;
        let t191 = rmath::sqrt(t190);
        let t193 = 9.0 * t157 + t189;
        let t194 = rmath::sqrt(t193);
        let t196 = t191 / 3.0 - t194 / 3.0;
        let t200 = t24 * t27;
        let t202 = t21 * t200 * t28;
        let t204 = t202 / 3.0 + t191 / 3.0;
        let t206 = t202 / 3.0 + t108;
        let t207 = 1.0 / t206;
        let t209 = rmath::ln(t204 * t207);
        let t213 = t202 / 3.0 + t194 / 3.0;
        let t215 = rmath::ln(t213 * t207);
        let t218 = 0.757211 + 0.04727288888888889 * t113 * t115 + 0.026366444444444446 * t138 * t140 - t184 * t185 / 9.0 + 2.0 / 3.0 * t25 * t29 * t196 + 2.0 * t94 * t209 - 2.0 * t157 * t215;
        let t222 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t218);
        let tzk0 = 2.0 * t222;
        zk[ip] += tzk0;
    }
}
