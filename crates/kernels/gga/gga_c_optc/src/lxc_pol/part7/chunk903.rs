//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 903/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk903<F: Float>(t1198: F, t481: F, t1205: F, t2887: F, t2900: F, t8639: F, t8642: F, t8587: F, t8589: F, t8591: F, t8622: F, t8625: F, t8628: F, t8630: F, t8632: F, t8636: F) -> (F, F, F, F, F, F) {
    let t9302 = t1198 * t1198;
    let t9303 = 1.0 / t9302;
    let t9304 = t481 * t9303;
    let t9305 = t2887 * t1205;
    let t9308 = t1205 * t2900;
    let t9311 = 0.22615185185185185185e4 * t8639;
    let t9312 = 0.34962962962962962963e3 * t8642;
    let t9322 = -t9311 - t9312 - 0.80768518518518518518e3 * t8622 - 0.72691666666666666667e3 * t8625 - 0.26222222222222222223e3 * t8628 + 0.15733333333333333334e3 * t8630 + 0.52444444444444444444e2 * t8632 - 0.34962962962962962963e2 * t8636 - 0.78666666666666666667e2 * t8587 - 0.96922222222222222223e3 * t8589 + 0.72691666666666666668e3 * t8591;
    (t9302, t9303, t9304, t9305, t9308, t9322)
}
