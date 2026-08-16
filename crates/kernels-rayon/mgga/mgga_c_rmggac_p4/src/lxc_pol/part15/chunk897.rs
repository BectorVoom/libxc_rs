//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 897/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk897(t1494: f64, t1970: f64, t1971: f64, t209: f64, t515: f64, t570: f64, t352: f64, t6172: f64, t118: f64, t128: f64, t1888: f64, t1986: f64) -> (f64, f64, f64) {
    let t45032 = t1970 * t1971 * t515 * t570 * t1494 * t209;
    let t45038 = t1970 * t1971 * t515 * t6172 * t352;
    let t45043 = t1986 * t118 * t128 * t1888 * t209;
    (t45032, t45038, t45043)
}
