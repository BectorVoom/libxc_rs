//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 654/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk654(t3104: f64, t5328: f64, t3109: f64, t3133: f64, t3151: f64, t4570: f64, t894: f64) -> (f64, f64, f64, f64, f64) {
    let t5329 = t3104 * t5328;
    let t5330 = t5329 * t3109;
    let t5333 = t5329 * t3133;
    let t5336 = t3151 * t4570;
    let t5337 = t894 * t5336;
    (t5329, t5330, t5333, t5336, t5337)
}
