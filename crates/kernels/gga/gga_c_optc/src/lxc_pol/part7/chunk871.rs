//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 871/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk871<F: Float>(t1065: F, t2972: F, t393: F, t2975: F, t401: F, t8787: F, t1085: F, t3029: F, t8639: F, t8589: F, t8591: F, t8593: F, t8603: F, t8606: F, t8609: F, t8622: F, t8625: F, t8657: F, t8660: F) -> (F, F, F, F, F, F) {
    let t8847 = 1.0 / t2972 / t1065;
    let t8848 = t393 * t8847;
    let t8850 = 1.0 / t2975 / t401;
    let t8851 = t8787 * t8850;
    let t8854 = t3029 * t1085;
    let t8857 = 0.28842592592592592592e-1 * t8639;
    let t8868 = -t8857 - 0.12361111111111111111e-1 * t8589 + 0.61805555555555555556e-2 * t8593 - 0.18541666666666666667e-1 * t8603 + 0.92708333333333333334e-2 * t8591 - 0.10300925925925925926e-1 * t8622 + 0.37083333333333333333e-1 * t8606 - 0.18541666666666666666e-1 * t8657 - 0.55625000000000000001e-1 * t8609 + 0.55625000000000000001e-1 * t8660 - 0.92708333333333333333e-2 * t8625;
    (t8847, t8848, t8850, t8851, t8854, t8868)
}
