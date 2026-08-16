//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1150/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1150(t23882: f64, t23897: f64, t23918: f64, t23940: f64, t828: f64, t837: f64, t845: f64, t549: f64, t6541: f64) -> (f64, f64, f64) {
    let t23942 = t23882 + t23897 + t23918 + t23940;
    let t23946 = 0.58482233974552040708e0_f64 * t845 * t828 * t23942 * t837;
    let t23951 = t6541 * t549;
    (t23942, t23946, t23951)
}
