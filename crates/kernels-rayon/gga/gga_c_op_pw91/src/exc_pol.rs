//! GGA_C_OP_PW91 exc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pw91.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_2};

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_op_pw91_exc_pol(
    rho: &[f64],
    sigma: &[f64],
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
        let t1 = rho0 - rho1;
        let t2 = rho0 + rho1;
        let t3 = 1.0 / t2;
        let t4 = t1 * t3;
        let t5 = rmath::abs(t4);
        let t11 = 1.0 - t5 <= zeta_threshold || rho0 <= dens_threshold && rho1 <= dens_threshold;
        let t13 = 1.0 + t4 <= zeta_threshold;
        let t14 = zeta_threshold - 1.0;
        let t16 = 1.0 - t4 <= zeta_threshold;
        let t17 = -t14;
        let t18 = piecewise5(t13, t14, t16, t17, t4);
        let t19 = t18 * t18;
        let t20 = 1.0 - t19;
        let t21 = t20 * t2;
        let t24 = 2.0 * rho0 * t3 <= zeta_threshold;
        let t27 = 2.0 * rho1 * t3 <= zeta_threshold;
        let t28 = piecewise5(t24, t14, t27, t17, t4);
        let t29 = 1.0 + t28;
        let t32 = t29 * t2 / 2.0 <= dens_threshold;
        let t33 = M_CBRT3;
        let t34 = t33 * t33;
        let t36 = pow_1_3(1.0 / M_PI);
        let t38 = t34 / t36;
        let t39 = M_CBRT4;
        let t40 = t38 * t39;
        let t41 = M_CBRT2;
        let t42 = t29 <= zeta_threshold;
        let t43 = 1.0 - t28;
        let t44 = t43 <= zeta_threshold;
        let t45 = piecewise5(t42, t14, t44, t17, t28);
        let t46 = 1.0 + t45;
        let t47 = t46 * t2;
        let t48 = pow_1_3(t47);
        let t49 = 1.0 / t48;
        let t50 = t41 * t49;
        let t51 = M_CBRT6;
        let t52 = M_PI * M_PI;
        let t53 = pow_1_3(t52);
        let t54 = t53 * t53;
        let t55 = 1.0 / t54;
        let t56 = t51 * t55;
        let t57 = rho0 * rho0;
        let t58 = pow_1_3(rho0);
        let t59 = t58 * t58;
        let t61 = 1.0 / t59 / t57;
        let t63 = t56 * sigma0 * t61;
        let t65 = rmath::exp(-25.0 / 6.0 * t63);
        let t68 = (0.2743 - 0.1508 * t65) * t51;
        let t69 = t55 * sigma0;
        let t73 = t51 * t51;
        let t75 = 1.0 / t53 / t52;
        let t76 = t73 * t75;
        let t77 = sigma0 * sigma0;
        let t78 = t57 * t57;
        let t79 = t78 * rho0;
        let t81 = 1.0 / t58 / t79;
        let t84 = 6.944444444444445e-06 * t76 * t77 * t81;
        let t85 = t68 * t69 * t61 / 24.0 - t84;
        let t87 = t73 / t53;
        let t88 = rmath::sqrt(sigma0);
        let t90 = 1.0 / t58 / rho0;
        let t91 = t88 * t90;
        let t94 = rmath::ln(0.6496333333333333 * t87 * t91 + rmath::sqrt(pow_2(0.6496333333333333 * t87 * t91) + 1.0));
        let t98 = 1.0 + 0.016370833333333334 * t87 * t91 * t94 + t84;
        let t99 = 1.0 / t98;
        let t101 = t85 * t99 + 1.0;
        let t102 = 1.0 / t101;
        let t106 = piecewise3(t32, 0.0, t40 * t50 * t102 / 9.0);
        let t110 = t43 * t2 / 2.0 <= dens_threshold;
        let t111 = piecewise5(t44, t14, t42, t17, -t28);
        let t112 = 1.0 + t111;
        let t113 = t112 * t2;
        let t114 = pow_1_3(t113);
        let t115 = 1.0 / t114;
        let t116 = t41 * t115;
        let t117 = rho1 * rho1;
        let t118 = pow_1_3(rho1);
        let t119 = t118 * t118;
        let t121 = 1.0 / t119 / t117;
        let t123 = t56 * sigma2 * t121;
        let t125 = rmath::exp(-25.0 / 6.0 * t123);
        let t128 = (0.2743 - 0.1508 * t125) * t51;
        let t129 = t55 * sigma2;
        let t133 = sigma2 * sigma2;
        let t134 = t117 * t117;
        let t135 = t134 * rho1;
        let t137 = 1.0 / t118 / t135;
        let t140 = 6.944444444444445e-06 * t76 * t133 * t137;
        let t141 = t128 * t129 * t121 / 24.0 - t140;
        let t142 = rmath::sqrt(sigma2);
        let t144 = 1.0 / t118 / rho1;
        let t145 = t142 * t144;
        let t148 = rmath::ln(0.6496333333333333 * t87 * t145 + rmath::sqrt(pow_2(0.6496333333333333 * t87 * t145) + 1.0));
        let t152 = 1.0 + 0.016370833333333334 * t87 * t145 * t148 + t140;
        let t153 = 1.0 / t152;
        let t155 = t141 * t153 + 1.0;
        let t156 = 1.0 / t155;
        let t160 = piecewise3(t110, 0.0, t40 * t116 * t156 / 9.0);
        let t161 = t106 + t160;
        let t162 = t161 == 0.0;
        let t163 = piecewise3(t162, f64::EPSILON, t161);
        let t166 = 3.60663084 / t163 + 0.5764;
        let t167 = t163 * t163;
        let t168 = t167 * t167;
        let t169 = 1.0 / t168;
        let t171 = t167 * t163;
        let t172 = 1.0 / t171;
        let t174 = 1.0 / t167;
        let t176 = 31.58152667175181 * t169 + 15.032732091624375 * t172 + 1.788764629788 * t174;
        let t177 = 1.0 / t176;
        let t178 = t166 * t177;
        let tzk0 = piecewise3(t11, 0.0, -0.25 * t21 * t178);
        zk[ip] += tzk0;
    }
}
