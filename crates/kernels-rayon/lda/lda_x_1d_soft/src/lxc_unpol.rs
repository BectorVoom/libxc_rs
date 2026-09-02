//! LDA_X_1D_SOFT lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_1d_soft.c`
//! by tools/translate_rayon/from_maple.py. Preserves maple2c's exact
//! variable names and floating-point operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::all)]

use libxc_rkernel_math::rmath;
use libxc_rkernel_math::constants::{M_PI};
use libxc_rkernel_math::bessel::{xc_bessel_K0, xc_bessel_K1};
use libxc_rkernel_math::integrate::{xc_integrate_lda_soft_func1, xc_integrate_lda_soft_func2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn lda_x_1d_soft_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    for ip in 0..zk.len() {
        let t3 = 1.0 <= zeta_threshold;
        let t4 = rho[ip] / 2.0 <= dens_threshold || t3;
        let t5 = zeta_threshold - 1.0;
        let t7 = piecewise5(t3, t5, t3, -t5, 0.0);
        let t8 = 1.0 + t7;
        let t11 = t8 * M_PI * param_beta * rho[ip];
        let t12 = xc_integrate_lda_soft_func1(t11);
        let t14 = xc_integrate_lda_soft_func2(t11);
        let t15 = 1.0 / M_PI;
        let t16 = t14 * t15;
        let t17 = 1.0 / param_beta;
        let t18 = 1.0 / rho[ip];
        let t19 = t17 * t18;
        let t24 = piecewise3(t4, 0.0, -0.07957747154594767 * (t8 * t12 - t16 * t19) * t17);
        let tzk0 = 2.0 * t24;
        zk[ip] += tzk0;
        let t25 = param_beta * param_beta;
        let t26 = 1.0 / t25;
        let t27 = rho[ip] * rho[ip];
        let t28 = 1.0 / t27;
        let t32 = piecewise3(t4, 0.0, -0.07957747154594767 * t16 * t26 * t28);
        let tvrho0 = 2.0 * rho[ip] * t32 + 2.0 * t24;
        vrho[ip] += tvrho0;
        let t36 = t8 * t8;
        let t37 = xc_bessel_K0( t11);
        let t38 = t36 * t37;
        let t42 = 1.0 / t27 / rho[ip];
        let t47 = piecewise3(t4, 0.0, -0.5 * t38 * t18 + 0.15915494309189535 * t16 * t26 * t42);
        let tv2rho20 = 2.0 * rho[ip] * t47 + 4.0 * t32;
        v2rho2[ip] += tv2rho20;
        let t52 = xc_bessel_K1( t11);
        let t53 = t36 * t8 * t52;
        let t54 = M_PI * param_beta;
        let t60 = t27 * t27;
        let t66 = piecewise3(t4, 0.0, 0.5 * t53 * t54 * t18 + 1.5 * t38 * t28 - 0.477464829275686 * t16 * t26 / t60);
        let tv3rho30 = 2.0 * rho[ip] * t66 + 6.0 * t47;
        v3rho3[ip] += tv3rho30;
        let t70 = t36 * t36;
        let t77 = M_PI * M_PI;
        let t93 = piecewise3(t4, 0.0, 0.5 * t70 * (-t37 - 1.0 / t8 * t15 * t19 * t52) * t77 * t25 * t18 - 2.0 * t53 * t54 * t28 - 6.0 * t38 * t42 + 1.909859317102744 * t16 * t26 / t60 / rho[ip]);
        let tv4rho40 = 2.0 * rho[ip] * t93 + 8.0 * t66;
        v4rho4[ip] += tv4rho40;
    }
}
