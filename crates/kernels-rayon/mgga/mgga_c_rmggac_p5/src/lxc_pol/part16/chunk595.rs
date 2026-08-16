//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 595/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk595(t515: f64, t8435: f64, t1971: f64, t1970: f64, t128: f64, t605: f64, t209: f64, t118: f64, t1986: f64) -> (f64, f64, f64) {
    let t8436 = t515 * t8435;
    let t8437 = t1971 * t8436;
    let t8438 = t1970 * t8437;
    let t8440 = t128 * t605;
    let t8441 = t8440 * t209;
    let t8442 = t118 * t8441;
    let t8443 = t1986 * t8442;
    (t8437, t8438, t8443)
}
