//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 868/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk868(t236: f64, t498: f64, t6108: f64, t7231: f64, t7365: f64, t321: f64, t3352: f64, t1971: f64, t333: f64, t511: f64, t352: f64, t515: f64) -> (f64, f64, f64, f64) {
    let t44600 = t7365 * t7231 * t236 * t6108 * t498;
    let t44605 = t7365 * t3352 * t236 * t6108 * t321;
    let t44610 = t7365 * t1971 * t511 * t6108 * t333;
    let t44615 = t7365 * t1971 * t515 * t6108 * t352;
    (t44600, t44605, t44610, t44615)
}
