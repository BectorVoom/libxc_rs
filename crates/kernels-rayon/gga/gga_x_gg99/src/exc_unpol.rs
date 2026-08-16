//! GGA_X_GG99 exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_gg99.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRTPI, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_2};
use libxc_rkernel_math::special::{xc_dilogarithm};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_gg99_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = M_CBRT3;
        let t4 = t3 * t3;
        let t5 = M_CBRTPI;
        let t8 = t4 / t5 / M_PI;
        let t9 = 1.0 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t12 = piecewise5(t9, t10, t9, -t10, 0.0);
        let t13 = 1.0 + t12;
        let t15 = pow_1_3(zeta_threshold);
        let t17 = pow_1_3(t13);
        let t19 = piecewise3(t13 <= zeta_threshold, t15 * zeta_threshold, t17 * t13);
        let t20 = pow_1_3(rho[ip]);
        let t21 = t19 * t20;
        let t22 = t8 * t21;
        let t23 = M_PI * M_PI;
        let t24 = f64::sqrt(sigma[ip]);
        let t25 = M_CBRT2;
        let t26 = t24 * t25;
        let t28 = 1.0 / t20 / rho[ip];
        let t29 = t26 * t28;
        let t30 = M_CBRT4;
        let t31 = f64::sqrt(3.0);
        let t32 = t23 * M_PI;
        let t33 = t31 * t32;
        let t34 = pow_1_3(t33);
        let t35 = t30 * t34;
        let t36 = t29 < t35;
        let t37 = pow_1_4(3.0);
        let t38 = M_SQRT2;
        let t39 = t37 * t38;
        let t40 = f64::sqrt(M_PI);
        let t42 = 1.0 / t40 / M_PI;
        let t43 = t39 * t42;
        let t44 = t35 - 0.1e-9;
        let t45 = t44 < t29;
        let t46 = piecewise3(t45, t44, t29);
        let t47 = t46 * t46;
        let t49 = t23 * t23;
        let t50 = t49 * t23;
        let t52 = t47 * t47;
        let t53 = t52 * t47;
        let t54 = 48.0 * t50 - t53;
        let t55 = f64::sqrt(t54);
        let t56 = 4.0 * t33 + t55;
        let t57 = pow_1_3(t56);
        let t58 = t57 * t57;
        let t59 = t47 + t58;
        let t60 = f64::sqrt(t59);
        let t62 = f64::powf(t56, 1.0 / 6.0);
        let t63 = 1.0 / t62;
        let t67 = f64::ln(t43 * t46 * t60 * t63 / 4.0 + f64::sqrt(pow_2(t43 * t46 * t60 * t63 / 4.0) + 1.0));
        let t68 = 1.0 / M_PI;
        let t69 = t35 + 0.1e-9;
        let t70 = t69 < t29;
        let t71 = piecewise3(t70, t29, t69);
        let t72 = t71 * t71;
        let t73 = t72 * t71;
        let t74 = t73 * t31;
        let t76 = t72 * t72;
        let t77 = t76 * t72;
        let t80 = 3.0 / t50 * t77 - 144.0;
        let t81 = f64::sqrt(t80);
        let t83 = f64::atan(t81 / 12.0);
        let t84 = t83 / 3.0;
        let t85 = f64::cos(t84);
        let t86 = t68 * t85;
        let t87 = t74 * t86;
        let t88 = f64::sqrt(t87);
        let t91 = f64::ln(t68 * t88 / 2.0 + f64::sqrt(pow_2(t68 * t88 / 2.0) + 1.0));
        let t92 = piecewise3(t36, t67, t91);
        let t94 = f64::exp(-2.0 * t92);
        let t95 = 1.0 + t94;
        let t96 = f64::ln(t95);
        let t99 = xc_dilogarithm(-t94);
        let t101 = -12.0 * t92 * t96 + t23 + 12.0 * t99;
        let t102 = 1.0 / t92;
        let t104 = 1.0 / f64::cosh(t92);
        let t105 = pow_1_3(t104);
        let t106 = t105 * t105;
        let t107 = 1.0 / t106;
        let t108 = pow_1_3(t68);
        let t109 = 1.0 / t108;
        let t111 = t107 * t109 * t30;
        let t112 = t101 * t102 * t111;
        let t115 = piecewise3(t2, 0.0, -t22 * t112 / 24.0);
        let tzk0 = 2.0 * t115;
        zk[ip] += tzk0;
    }
}
