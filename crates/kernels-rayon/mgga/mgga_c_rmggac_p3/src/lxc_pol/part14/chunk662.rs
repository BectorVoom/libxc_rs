//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 662/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk662(t8728: f64, t8757: f64, t8778: f64, t8792: f64, t515: f64, t235: f64, t2367: f64, t874: f64, t352: f64, t1356: f64, t570: f64, t7567: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8794 = t8728 + t8757 + t8778 + t8792;
    let t8795 = t515 * t8794;
    let t8796 = t235 * t8795;
    let t8800 = t874 * t2367;
    let t8801 = t8800 * t352;
    let t8802 = t1356 * t8801;
    let t8804 = t7567 * t570;
    (t8794, t8795, t8796, t8800, t8801, t8802, t8804)
}
