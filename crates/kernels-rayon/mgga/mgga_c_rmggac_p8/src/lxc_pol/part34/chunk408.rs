//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 408/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk408(t1469: f64, t236: f64, t1475: f64, t498: f64, t321: f64, t333: f64, t511: f64, t352: f64, t515: f64, t128: f64, t605: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8416 = t236 * t1469;
    let t8420 = t1475 * t498;
    let t8421 = t236 * t8420;
    let t8425 = t1475 * t321;
    let t8426 = t236 * t8425;
    let t8430 = t1475 * t333;
    let t8431 = t511 * t8430;
    let t8435 = t1475 * t352;
    let t8436 = t515 * t8435;
    let t8440 = t128 * t605;
    let t8441 = t8440 * t209;
    (t8416, t8420, t8421, t8425, t8426, t8430, t8431, t8435, t8436, t8440, t8441)
}
