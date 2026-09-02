//! LDA_X_1D_SOFT vxc pol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_soft.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI};
use libxc_rkernel_math::integrate::{xc_integrate_lda_soft_func1, xc_integrate_lda_soft_func2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_1d_soft_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let t2 = rho0 - rho1;
        let t3 = rho0 + rho1;
        let t4 = 1.0 / t3;
        let t5 = t2 * t4;
        let t7 = 1.0 + t5 <= zeta_threshold;
        let t8 = rho0 <= dens_threshold || t7;
        let t9 = zeta_threshold - 1.0;
        let t11 = 1.0 - t5 <= zeta_threshold;
        let t12 = -t9;
        let t13 = piecewise5(t7, t9, t11, t12, t5);
        let t14 = 1.0 + t13;
        let t15 = t14 * M_PI;
        let t16 = param_beta * t3;
        let t17 = t15 * t16;
        let t18 = xc_integrate_lda_soft_func1(t17);
        let t20 = xc_integrate_lda_soft_func2(t17);
        let t21 = 1.0 / M_PI;
        let t22 = t20 * t21;
        let t23 = 1.0 / param_beta;
        let t24 = t23 * t4;
        let t29 = piecewise3(t8, 0.0, -0.07957747154594767 * (t14 * t18 - t22 * t24) * t23);
        let t31 = rho1 <= dens_threshold || t11;
        let t32 = piecewise5(t11, t9, t7, t12, -t5);
        let t33 = 1.0 + t32;
        let t34 = t33 * M_PI;
        let t35 = t34 * t16;
        let t36 = xc_integrate_lda_soft_func1(t35);
        let t38 = xc_integrate_lda_soft_func2(t35);
        let t39 = t38 * t21;
        let t44 = piecewise3(t31, 0.0, -0.07957747154594767 * (-t39 * t24 + t33 * t36) * t23);
        let tzk0 = t29 + t44;
        zk[ip] += tzk0;
        let t45 = t3 * t3;
        let t46 = 1.0 / t45;
        let t47 = t2 * t46;
        let t48 = t4 - t47;
        let t49 = piecewise5(t7, 0.0, t11, 0.0, t48);
        let t51 = t23 * t46;
        let t52 = t22 * t51;
        let t56 = piecewise3(t8, 0.0, -0.07957747154594767 * (t49 * t18 + t52) * t23);
        let t58 = piecewise5(t11, 0.0, t7, 0.0, -t48);
        let t60 = t39 * t51;
        let t64 = piecewise3(t31, 0.0, -0.07957747154594767 * (t58 * t36 + t60) * t23);
        let tvrho0 = t29 + t44 + t3 * (t56 + t64);
        vrho[ip * 2] += tvrho0;
        let t67 = -t4 - t47;
        let t68 = piecewise5(t7, 0.0, t11, 0.0, t67);
        let t73 = piecewise3(t8, 0.0, -0.07957747154594767 * (t68 * t18 + t52) * t23);
        let t75 = piecewise5(t11, 0.0, t7, 0.0, -t67);
        let t80 = piecewise3(t31, 0.0, -0.07957747154594767 * (t75 * t36 + t60) * t23);
        let tvrho1 = t29 + t44 + t3 * (t73 + t80);
        vrho[ip * 2 + 1] += tvrho1;
    }
}
