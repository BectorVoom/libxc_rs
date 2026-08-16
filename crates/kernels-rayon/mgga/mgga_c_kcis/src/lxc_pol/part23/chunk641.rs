//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 641/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk641(t1616: f64, t6188: f64, t1592: f64, t3729: f64, t3731: f64, t4143: f64, t5418: f64, t5630: f64, t5635: f64, t5639: f64, t5646: f64, t5651: f64, t5657: f64, t5665: f64, t5669: f64, t5674: f64, t5679: f64, t5681: f64, t5684: f64, t5686: f64, t6136: f64, t626: f64) -> (f64, f64) {
    let t6189 = t6188 * t1616;
    let t6192 = -0.17411041666666666666e-2_f64 * t5418 - 0.17411041666666666666e-2_f64 * t5630 + 0.46429444444444444443e-2_f64 * t5635 + 0.77382407407407407407e-3_f64 * t5639 - 0.11607361111111111111e-2_f64 * t3729 + 0.77382407407407407407e-3_f64 * t3731 - 0.11607361111111111111e-2_f64 * t5646 + 0.77382407407407407407e-3_f64 * t5651 - 0.23214722222222222222e-2_f64 * t5657 + 0.19345601851851851852e-2_f64 * t5665 - 0.11607361111111111111e-2_f64 * t5669 - 0.11607361111111111111e-2_f64 * t5674 + 0.34822083333333333332e-2_f64 * t5679 + 0.77382407407407407407e-3_f64 * t5681 + 0.77382407407407407407e-3_f64 * t4143 - 0.30952962962962962962e-2_f64 * t5684 + 0.11607361111111111111e-2_f64 * t5686 + t6136 * t626 - 0.66725e-1_f64 * t1592 * t6189;
    (t6189, t6192)
}
