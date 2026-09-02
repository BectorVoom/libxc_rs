//! LDA_X_YUKAWA exc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_yukawa.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_yukawa_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t1 = M_CBRT3;
        let t3 = pow_1_3(1.0 / M_PI);
        let t5 = M_CBRT4;
        let t6 = t5 * t5;
        let t7 = t6 * t3 * t1;
        let t8 = M_CBRT2;
        let t9 = t8 * t8;
        let t10 = 1.0 <= zeta_threshold;
        let t11 = pow_1_3(zeta_threshold);
        let t13 = piecewise3(t10, t11 * zeta_threshold, 1.0);
        let t14 = t13 * t9;
        let t15 = pow_1_3(rho[ip]);
        let t16 = pow_1_3(9.0);
        let t17 = t16 * t16;
        let t18 = t3 * t3;
        let t20 = param_hyb_omega_0 * t18 * t17;
        let t23 = piecewise3(t10, t11, 1.0);
        let t24 = 1.0 / t23;
        let t27 = t24 / t15 * t1 * t20 / 18.0;
        let t28 = 1.92 <= t27;
        let t29 = 1.92 < t27;
        let t30 = piecewise3(t29, t27, 1.92);
        let t31 = t30 * t30;
        let t34 = t31 * t31;
        let t35 = 1.0 / t34;
        let t37 = t34 * t31;
        let t38 = 1.0 / t37;
        let t40 = t34 * t34;
        let t41 = 1.0 / t40;
        let t43 = t40 * t31;
        let t44 = 1.0 / t43;
        let t46 = t40 * t34;
        let t47 = 1.0 / t46;
        let t49 = t40 * t37;
        let t50 = 1.0 / t49;
        let t52 = t40 * t40;
        let t53 = 1.0 / t52;
        let t56 = 1.0 / t52 / t31;
        let t59 = 1.0 / t52 / t34;
        let t62 = 1.0 / t52 / t37;
        let t65 = 1.0 / t52 / t40;
        let t68 = 1.0 / t52 / t43;
        let t71 = 1.0 / t52 / t46;
        let t74 = 1.0 / t52 / t49;
        let t76 = t52 * t52;
        let t77 = 1.0 / t76;
        let t80 = 1.0 / t76 / t31;
        let t83 = 1.0 / t76 / t34;
        let t85 = 1.0 / t31 / 9.0 - t35 / 30.0 + t38 / 70.0 - t41 / 135.0 + t44 / 231.0 - t47 / 364.0 + t50 / 540.0 - t53 / 765.0 + t56 / 1045.0 - t59 / 1386.0 + t62 / 1794.0 - t65 / 2275.0 + t68 / 2835.0 - t71 / 3480.0 + t74 / 4216.0 - t77 / 5049.0 + t80 / 5985.0 - t83 / 7030.0;
        let t86 = piecewise3(t29, 1.92, t27);
        let t87 = rmath::atan2(1.0, t86);
        let t88 = t86 * t86;
        let t89 = t88 + 3.0;
        let t90 = 1.0 / t88;
        let t91 = 1.0 + t90;
        let t92 = rmath::ln(t91);
        let t94 = -t92 * t89 + 1.0;
        let t97 = t87 + t94 * t86 / 4.0;
        let t101 = piecewise3(t28, t85, 1.0 - 8.0 / 3.0 * t97 * t86);
        let t104 = t101 * t15 * t14 * t7;
        let tzk0 = -3.0 / 16.0 * t104;
        zk[ip] += tzk0;
    }
}
