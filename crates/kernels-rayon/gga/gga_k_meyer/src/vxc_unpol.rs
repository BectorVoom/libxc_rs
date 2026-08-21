//! GGA_K_MEYER vxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_meyer.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT6, M_CBRTPI, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_meyer_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t7 = t4 * t5 * M_PI;
        let t8 = 1.0 <= zeta_threshold;
        let t9 = zeta_threshold - 1.0;
        let t11 = piecewise5(t8, t9, t8, -t9, 0.0);
        let t12 = 1.0 + t11;
        let t14 = pow_1_3(zeta_threshold);
        let t15 = t14 * t14;
        let t17 = pow_1_3(t12);
        let t18 = t17 * t17;
        let t20 = piecewise3(t12 <= zeta_threshold, t15 * zeta_threshold, t18 * t12);
        let t21 = pow_1_3(rho[ip]);
        let t22 = t21 * t21;
        let t23 = t20 * t22;
        let t24 = M_CBRT6;
        let t25 = M_PI * M_PI;
        let t26 = pow_1_3(t25);
        let t27 = t26 * t26;
        let t28 = 1.0 / t27;
        let t29 = t24 * t28;
        let t30 = M_CBRT2;
        let t31 = t30 * t30;
        let t32 = sigma[ip] * t31;
        let t33 = rho[ip] * rho[ip];
        let t35 = 1.0 / t22 / t33;
        let t39 = 1.0 - t29 * t32 * t35 / 864.0;
        let t40 = t24 * t24;
        let t41 = 1.0 / t26;
        let t42 = t40 * t41;
        let t43 = rmath::sqrt(sigma[ip]);
        let t44 = t43 * t30;
        let t45 = t21 * rho[ip];
        let t46 = 1.0 / t45;
        let t49 = t42 * t44 * t46 / 72.0;
        let t50 = 1.0 + t49;
        let t51 = 1.0 - t49;
        let t52 = rmath::abs(t51);
        let t53 = 1.0 / t52;
        let t55 = rmath::ln(t50 * t53);
        let t57 = t39 * t55 * t24;
        let t58 = 1.0 / t43;
        let t59 = t26 * t58;
        let t60 = t31 * t45;
        let t63 = 3.0 / 2.0 * t57 * t59 * t60;
        let t64 = 1.0 / 2.0 - t63;
        let t65 = 1.0 / 2.0 + t63;
        let t66 = 1.0 / t65;
        let t69 = 20.0 * t64 * t66 + 1.0;
        let t73 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t69);
        let tzk0 = 2.0 * t73;
        zk[ip] += tzk0;
        let t75 = t20 / t21;
        let t79 = t42 * t43;
        let t81 = 1.0 / t21 / t33;
        let t82 = t30 * t81;
        let t83 = t82 * t55;
        let t86 = t82 * t53;
        let t88 = t52 * t52;
        let t89 = 1.0 / t88;
        let t90 = t50 * t89;
        let t91 = t90 * t42;
        let t92 = rmath::abs(t51) / t51;
        let t93 = t81 * t92;
        let t97 = -t91 * t44 * t93 / 54.0 - t79 * t86 / 54.0;
        let t98 = t39 * t97;
        let t99 = 1.0 / t50;
        let t100 = t99 * t52;
        let t101 = t98 * t100;
        let t102 = t24 * t26;
        let t103 = t58 * t31;
        let t105 = t102 * t103 * t45;
        let t108 = t31 * t21;
        let t112 = -t79 * t83 / 108.0 - 3.0 / 2.0 * t101 * t105 - 2.0 * t57 * t59 * t108;
        let t114 = t65 * t65;
        let t115 = 1.0 / t114;
        let t116 = t64 * t115;
        let t117 = -t112;
        let t120 = 20.0 * t112 * t66 - 20.0 * t116 * t117;
        let t125 = piecewise3(t2, 0.0, t7 * t75 * t69 / 10.0 + 3.0 / 20.0 * t7 * t23 * t120);
        let tvrho0 = 2.0 * rho[ip] * t125 + 2.0 * t73;
        vrho[ip] += tvrho0;
        let t128 = t42 * t30;
        let t129 = t46 * t55;
        let t133 = t42 * t58;
        let t134 = t30 * t46;
        let t135 = t134 * t53;
        let t137 = t58 * t30;
        let t138 = t46 * t92;
        let t142 = t91 * t137 * t138 / 144.0 + t133 * t135 / 144.0;
        let t143 = t39 * t142;
        let t144 = t143 * t100;
        let t147 = t43 * sigma[ip];
        let t148 = 1.0 / t147;
        let t149 = t26 * t148;
        let t153 = t128 * t129 * t58 / 288.0 - 3.0 / 2.0 * t144 * t105 + 3.0 / 4.0 * t57 * t149 * t60;
        let t155 = -t153;
        let t158 = -20.0 * t116 * t155 + 20.0 * t153 * t66;
        let t162 = piecewise3(t2, 0.0, 3.0 / 20.0 * t7 * t23 * t158);
        let tvsigma0 = 2.0 * rho[ip] * t162;
        vsigma[ip] += tvsigma0;
    }
}
