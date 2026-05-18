//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 897/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk897<F: Float>(t1616: F, t6188: F, t1592: F, t3729: F, t3731: F, t4143: F, t5418: F, t5630: F, t5635: F, t5639: F, t5646: F, t5651: F, t5657: F, t5665: F, t5669: F, t5674: F, t5679: F, t5681: F, t5684: F, t5686: F, t6136: F, t626: F) -> (F, F) {
    let t6189 = t6188 * t1616;
    let t6192 = -F::new(0.17411041666666666666e-2) * t5418 - F::new(0.17411041666666666666e-2) * t5630 + F::new(0.46429444444444444443e-2) * t5635 + F::new(0.77382407407407407407e-3) * t5639 - F::new(0.11607361111111111111e-2) * t3729 + F::new(0.77382407407407407407e-3) * t3731 - F::new(0.11607361111111111111e-2) * t5646 + F::new(0.77382407407407407407e-3) * t5651 - F::new(0.23214722222222222222e-2) * t5657 + F::new(0.19345601851851851852e-2) * t5665 - F::new(0.11607361111111111111e-2) * t5669 - F::new(0.11607361111111111111e-2) * t5674 + F::new(0.34822083333333333332e-2) * t5679 + F::new(0.77382407407407407407e-3) * t5681 + F::new(0.77382407407407407407e-3) * t4143 - F::new(0.30952962962962962962e-2) * t5684 + F::new(0.11607361111111111111e-2) * t5686 + t6136 * t626 - F::new(0.66725e-1) * t1592 * t6189;
    (t6189, t6192)
}
