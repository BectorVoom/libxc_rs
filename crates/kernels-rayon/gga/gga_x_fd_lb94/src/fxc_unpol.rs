//! GGA_X_FD_LB94 fxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_fd_lb94.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::integrate::{xc_integrate_func0, xc_integrate_func1};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_fd_lb94_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_beta: f64,
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
        let t19 = t17 * t18;
        let t20 = M_CBRT6;
        let t21 = t20 * t20;
        let t22 = M_PI * M_PI;
        let t23 = pow_1_3(t22);
        let t24 = 1.0 / t23;
        let t25 = t21 * t24;
        let t26 = f64::sqrt(sigma[ip]);
        let t27 = t25 * t26;
        let t28 = M_CBRT2;
        let t30 = 1.0 / t18 / rho[ip];
        let t31 = t28 * t30;
        let t35 = t25 * t26 * t28 * t30 / 12.0;
        let t36 = xc_integrate_func0(t35, param_beta);
        let t37 = f64::ln(t35);
        let t39 = xc_integrate_func1(t35, param_beta);
        let t40 = t36 * t37 - t39;
        let t41 = t31 * t40;
        let t44 = 1.0 - t27 * t41 / 12.0;
        let t48 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t44);
        let tzk0 = 2.0 * t48;
        zk[ip] += tzk0;
        let t49 = t18 * t18;
        let t51 = t17 / t49;
        let t55 = rho[ip] * rho[ip];
        let t57 = 1.0 / t18 / t55;
        let t58 = t28 * t57;
        let t59 = t58 * t40;
        let t61 = t58 * t36;
        let t64 = t27 * t59 / 9.0 + t27 * t61 / 9.0;
        let t69 = piecewise3(t2, 0.0, -t6 * t51 * t44 / 8.0 - 3.0 / 8.0 * t6 * t19 * t64);
        let tvrho0 = 2.0 * rho[ip] * t69 + 2.0 * t48;
        vrho[ip] += tvrho0;
        let t72 = 1.0 / t26;
        let t73 = t25 * t72;
        let t75 = t31 * t36;
        let t78 = -t73 * t41 / 24.0 - t73 * t75 / 24.0;
        let t82 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t78);
        let tvsigma0 = 2.0 * rho[ip] * t82;
        vsigma[ip] += tvsigma0;
        let t87 = t17 / t49 / rho[ip];
        let t94 = t55 * rho[ip];
        let t96 = 1.0 / t18 / t94;
        let t97 = t28 * t96;
        let t98 = t97 * t40;
        let t101 = t97 * t36;
        let t104 = t23 * t23;
        let t105 = 1.0 / t104;
        let t106 = t20 * t105;
        let t107 = t106 * sigma[ip];
        let t108 = t55 * t55;
        let t110 = 1.0 / t49 / t108;
        let t111 = t110 * param_beta;
        let t112 = t28 * t28;
        let t113 = param_beta * t112;
        let t114 = t113 * t21;
        let t115 = t24 * t26;
        let t116 = t112 * t21;
        let t121 = t105 * sigma[ip];
        let t123 = 1.0 / t49 / t55;
        let t127 = 3.0 * t28 * t20 * t121 * t123 + 36.0;
        let t128 = f64::sqrt(t127);
        let t130 = t116 * t115 * t30 / 12.0 + t128 / 6.0;
        let t131 = f64::ln(t130);
        let t132 = t30 * t131;
        let t136 = 1.0 + t114 * t115 * t132 / 4.0;
        let t137 = 1.0 / t136;
        let t138 = t111 * t137;
        let t141 = -7.0 / 27.0 * t27 * t98 - 11.0 / 27.0 * t27 * t101 + t107 * t138 / 9.0;
        let t146 = piecewise3(t2, 0.0, t6 * t87 * t44 / 12.0 - t6 * t51 * t64 / 4.0 - 3.0 / 8.0 * t6 * t19 * t141);
        let tv2rho20 = 2.0 * rho[ip] * t146 + 4.0 * t69;
        v2rho2[ip] += tv2rho20;
        let t157 = 1.0 / t49 / t94;
        let t158 = t157 * param_beta;
        let t159 = t158 * t137;
        let t162 = t73 * t59 / 18.0 + t73 * t61 / 9.0 - t106 * t159 / 24.0;
        let t167 = piecewise3(t2, 0.0, -t6 * t51 * t78 / 8.0 - 3.0 / 8.0 * t6 * t19 * t162);
        let tv2rhosigma0 = 2.0 * rho[ip] * t167 + 2.0 * t82;
        v2rhosigma[ip] += tv2rhosigma0;
        let t171 = 1.0 / t26 / sigma[ip];
        let t172 = t25 * t171;
        let t175 = 1.0 / sigma[ip];
        let t176 = t106 * t175;
        let t177 = t123 * param_beta;
        let t178 = t177 * t137;
        let t181 = t172 * t41 / 48.0 + t176 * t178 / 64.0;
        let t185 = piecewise3(t2, 0.0, -3.0 / 8.0 * t6 * t19 * t181);
        let tv2sigma20 = 2.0 * rho[ip] * t185;
        v2sigma2[ip] += tv2sigma20;
    }
}
