//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 676/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk676<F: Float>(t2823: F, t2862: F, t3052: F, t3174: F, t430: F, t4550: F, t4558: F, t4775: F, t4779: F, t4787: F, t4790: F, t4794: F, t4798: F, t4803: F, t4808: F, t4816: F, t4821: F, t4826: F, t4926: F, t5272: F) -> (F,) {
    let t5280 = -0.23214722222222222222e-2 * t4550 + 0.19345601851851851852e-2 * t4558 - 0.17411041666666666666e-2 * t4775 + 0.77382407407407407407e-3 * t2823 - 0.17411041666666666666e-2 * t4779 + 0.11607361111111111111e-2 * t4787 - 0.46429444444444444443e-2 * t4790 - 0.11607361111111111111e-2 * t4794 + 0.77382407407407407407e-3 * t4798 - 0.11607361111111111111e-2 * t4803 + 0.77382407407407407407e-3 * t4808 - 0.11607361111111111111e-2 * t2862 + t5272 * t430 + 0.11607361111111111111e-2 * t3052 + 0.77382407407407407407e-3 * t3174 - 0.38691203703703703703e-3 * t4816 + 0.34822083333333333332e-2 * t4821 - 0.11607361111111111111e-2 * t4826 + 0.17411041666666666666e-2 * t4926;
    (t5280,)
}
