//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 968/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk968<F: Float>(t8639: F, t8642: F, t8587: F, t8589: F, t8591: F, t8622: F, t8625: F, t8628: F, t8630: F, t8632: F, t8636: F, t8593: F, t8595: F, t8598: F, t8601: F, t8603: F, t8606: F, t8609: F, t8651: F, t8654: F, t8657: F, t8660: F) -> (F, F) {
    let t9268 = F::new(0.60319259259259259259e1) * t8639;
    let t9269 = F::new(0.54733333333333333333e-2) * t8642;
    let t9279 = -t9268 - t9269 - F::new(0.21542592592592592592e1) * t8622 - F::new(0.19388333333333333333e1) * t8625 - F::new(0.4105e-2) * t8628 + F::new(0.2463e-2) * t8630 + F::new(0.821e-3) * t8632 - F::new(0.54733333333333333333e-3) * t8636 - F::new(0.12315e-2) * t8587 - F::new(0.2585111111111111111e1) * t8589 + F::new(0.19388333333333333333e1) * t8591;
    let t9291 = F::new(0.12925555555555555555e1) * t8593 - F::new(0.4926e-2) * t8595 + F::new(0.2463e-2) * t8598 - F::new(0.12315e-2) * t8651 - F::new(0.7389e-2) * t8601 + F::new(0.7389e-2) * t8654 - F::new(0.38776666666666666665e1) * t8603 + F::new(0.77553333333333333331e1) * t8606 - F::new(0.38776666666666666665e1) * t8657 - F::new(0.11633e2) * t8609 + F::new(0.11633e2) * t8660;
    (t9279, t9291)
}
