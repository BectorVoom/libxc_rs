//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 629/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk629(t1970: f64, t8422: f64, t1475: f64, t321: f64, t236: f64, t3352: f64, t333: f64, t511: f64, t1971: f64, t352: f64, t515: f64, t128: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8423 = t1970 * t8422;
    let t8425 = t1475 * t321;
    let t8426 = t236 * t8425;
    let t8427 = t3352 * t8426;
    let t8428 = t1970 * t8427;
    let t8430 = t1475 * t333;
    let t8431 = t511 * t8430;
    let t8432 = t1971 * t8431;
    let t8433 = t1970 * t8432;
    let t8435 = t1475 * t352;
    let t8436 = t515 * t8435;
    let t8437 = t1971 * t8436;
    let t8438 = t1970 * t8437;
    let t8440 = t128 * t605;
    (t8423, t8427, t8428, t8432, t8433, t8437, t8438, t8440)
}
