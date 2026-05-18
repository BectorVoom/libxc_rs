//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 970/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk970<F: Float>(t8642: F, t8587: F, t8589: F, t8591: F, t8622: F, t8625: F, t8628: F, t8630: F, t8632: F, t8636: F, t9311: F, t8593: F, t8595: F, t8598: F, t8601: F, t8603: F, t8606: F, t8609: F, t8651: F, t8654: F, t8657: F, t8660: F) -> (F, F) {
    let t9312 = F::new(0.34962962962962962963e3) * t8642;
    let t9322 = -t9311 - t9312 - F::new(0.80768518518518518518e3) * t8622 - F::new(0.72691666666666666667e3) * t8625 - F::new(0.26222222222222222223e3) * t8628 + F::new(0.15733333333333333334e3) * t8630 + F::new(0.52444444444444444444e2) * t8632 - F::new(0.34962962962962962963e2) * t8636 - F::new(0.78666666666666666667e2) * t8587 - F::new(0.96922222222222222223e3) * t8589 + F::new(0.72691666666666666668e3) * t8591;
    let t9334 = F::new(0.48461111111111111112e3) * t8593 - F::new(0.31466666666666666667e3) * t8595 + F::new(0.15733333333333333333e3) * t8598 - F::new(0.78666666666666666666e2) * t8651 - F::new(0.47199999999999999999e3) * t8601 + F::new(0.47199999999999999999e3) * t8654 - F::new(0.14538333333333333333e4) * t8603 + F::new(0.29076666666666666666e4) * t8606 - F::new(0.14538333333333333333e4) * t8657 - F::new(0.43614999999999999999e4) * t8609 + F::new(0.43614999999999999999e4) * t8660;
    (t9322, t9334)
}
