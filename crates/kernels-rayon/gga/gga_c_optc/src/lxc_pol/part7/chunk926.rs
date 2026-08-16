//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 926/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk926(t1062: f64, t2934: f64, t1075: f64, t8787: f64, t8587: f64, t8589: f64, t8591: f64, t8593: f64, t8595: f64, t8598: f64, t8601: f64, t8603: f64, t8606: f64, t8609: f64, t8613: f64, t8618: f64, t8622: f64, t8625: f64) -> (f64, f64, f64) {
    let t8806 = t1062 * t2934;
    let t8809 = t8787 * t1075;
    let t8826 = -0.104195e0_f64 * t8587 - 0.68863333333333333332e0_f64 * t8589 + 0.51647499999999999999e0_f64 * t8591 + 0.34431666666666666666e0_f64 * t8593 - 0.41678000000000000001e0_f64 * t8595 + 0.20839e0_f64 * t8598 - 0.62517e0_f64 * t8601 - 0.103295e1_f64 * t8603 + 0.20659e1_f64 * t8606 - 0.309885e1_f64 * t8609 - 0.157790625e0_f64 * t8613 + 0.264729375e1_f64 * t8618 - 0.57386111111111111112e0_f64 * t8622 - 0.516475e0_f64 * t8625;
    (t8806, t8809, t8826)
}
