//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 750/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk750(t2333: f64, t7234: f64, t2367: f64, t2543: f64, t999: f64, t6541: f64, t769: f64) -> (f64, f64, f64, f64) {
    let t7235 = t2333 * t7234;
    let t7239 = t2367 * t2543;
    let t7240 = t999 * t7239;
    let t7244 = t769 * t6541;
    (t7235, t7239, t7240, t7244)
}
