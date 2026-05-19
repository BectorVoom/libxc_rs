//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 926/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk926<F: Float>(t1062: F, t2934: F, t1075: F, t8787: F, t8587: F, t8589: F, t8591: F, t8593: F, t8595: F, t8598: F, t8601: F, t8603: F, t8606: F, t8609: F, t8613: F, t8618: F, t8622: F, t8625: F) -> (F, F, F) {
    let t8806 = t1062 * t2934;
    let t8809 = t8787 * t1075;
    let t8826 = -F::new(0.104195e0) * t8587 - F::cast_from(0.68863333333333333332e0_f64) * t8589 + F::cast_from(0.51647499999999999999e0_f64) * t8591 + F::cast_from(0.34431666666666666666e0_f64) * t8593 - F::cast_from(0.41678000000000000001e0_f64) * t8595 + F::new(0.20839e0) * t8598 - F::new(0.62517e0) * t8601 - F::new(0.103295e1) * t8603 + F::new(0.20659e1) * t8606 - F::new(0.309885e1) * t8609 - F::cast_from(0.157790625e0_f64) * t8613 + F::cast_from(0.264729375e1_f64) * t8618 - F::cast_from(0.57386111111111111112e0_f64) * t8622 - F::new(0.516475e0) * t8625;
    (t8806, t8809, t8826)
}
