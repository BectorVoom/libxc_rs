//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 628/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk628(t8728: f64, t8757: f64, t8778: f64, t8792: f64, t515: f64, t235: f64, t2367: f64, t874: f64) -> (f64, f64, f64, f64) {
    let t8794 = t8728 + t8757 + t8778 + t8792;
    let t8795 = t515 * t8794;
    let t8796 = t235 * t8795;
    let t8800 = t874 * t2367;
    (t8794, t8795, t8796, t8800)
}
