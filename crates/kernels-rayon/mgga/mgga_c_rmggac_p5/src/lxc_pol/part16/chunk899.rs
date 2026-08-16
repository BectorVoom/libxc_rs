//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 899/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk899(t8365: f64, t8562: f64, t131: f64, t6344: f64, t638: f64, t639: f64, t71: f64, t356: f64, t9745: f64, t574: f64, t8849: f64, t1656: f64, t2338: f64) -> (f64, f64, f64, f64, f64) {
    let t44911 = t8365 * t8562;
    let t44916 = t638 * t639 * t71 * t6344 * t131;
    let t44920 = t638 * t639 * t9745 * t356;
    let t44925 = t638 * t639 * t8849 * t574;
    let t44929 = t638 * t639 * t2338 * t1656;
    (t44911, t44916, t44920, t44925, t44929)
}
