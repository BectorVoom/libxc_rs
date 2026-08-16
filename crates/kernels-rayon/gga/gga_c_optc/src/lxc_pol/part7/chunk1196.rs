//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1196/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1196(t24228: f64, t24230: f64, t24233: f64, t24299: f64, t24308: f64, t24337: f64, t24339: f64, t24344: f64, t24693: f64, t24696: f64, t24702: f64, t24704: f64) -> f64 {
    let t24705 = t24228 + t24230 + t24233 + t24299 + t24308 + t24337 + t24339 - t24344 - t24693 - t24696 - t24702 + t24704;
    t24705
}
