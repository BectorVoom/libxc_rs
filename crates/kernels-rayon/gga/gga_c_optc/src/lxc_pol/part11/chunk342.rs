//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 342/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk342(t1102: f64, t1492: f64, t422: f64, t424: f64, rho1: f64) -> (f64, f64, f64) {
    let t1494 = 0.58482233974552040708e0_f64 * t1102 * t1492;
    let t1495 = t422 * rho1;
    let t1497 = 1.0_f64 / t424 / t1495;
    (t1494, t1495, t1497)
}
