//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1231/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1231(t2812: f64, t8040: f64, t8143: f64, t2723: f64, t7178: f64, t311: f64, t8112: f64) -> (f64, f64, f64) {
    let t25417 = t2812 * t8143 * t8040;
    let t25419 = t7178 * t2723;
    let t25423 = t8112 * t311;
    (t25417, t25419, t25423)
}
