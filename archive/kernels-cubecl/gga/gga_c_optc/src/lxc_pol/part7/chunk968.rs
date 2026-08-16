//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 968/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk968<F: Float>(t8639: F, t8642: F, t8587: F, t8589: F, t8591: F, t8622: F, t8625: F, t8628: F, t8630: F, t8632: F, t8636: F, t8593: F, t8595: F, t8598: F, t8601: F, t8603: F, t8606: F, t8609: F, t8651: F, t8654: F, t8657: F, t8660: F) -> (F, F) {
    let t9268 = F::cast_from(0.60319259259259259259e1_f64) * t8639;
    let t9269 = F::cast_from(0.54733333333333333333e-2_f64) * t8642;
    let t9279 = -t9268 - t9269 - F::cast_from(0.21542592592592592592e1_f64) * t8622 - F::cast_from(0.19388333333333333333e1_f64) * t8625 - F::cast_from(0.4105e-2_f64) * t8628 + F::cast_from(0.2463e-2_f64) * t8630 + F::cast_from(0.821e-3_f64) * t8632 - F::cast_from(0.54733333333333333333e-3_f64) * t8636 - F::cast_from(0.12315e-2_f64) * t8587 - F::cast_from(0.2585111111111111111e1_f64) * t8589 + F::cast_from(0.19388333333333333333e1_f64) * t8591;
    let t9291 = F::cast_from(0.12925555555555555555e1_f64) * t8593 - F::cast_from(0.4926e-2_f64) * t8595 + F::cast_from(0.2463e-2_f64) * t8598 - F::cast_from(0.12315e-2_f64) * t8651 - F::cast_from(0.7389e-2_f64) * t8601 + F::cast_from(0.7389e-2_f64) * t8654 - F::cast_from(0.38776666666666666665e1_f64) * t8603 + F::cast_from(0.77553333333333333331e1_f64) * t8606 - F::cast_from(0.38776666666666666665e1_f64) * t8657 - F::cast_from(0.11633e2_f64) * t8609 + F::cast_from(0.11633e2_f64) * t8660;
    (t9279, t9291)
}
