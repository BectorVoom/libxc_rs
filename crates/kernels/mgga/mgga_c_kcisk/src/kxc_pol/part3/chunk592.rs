//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 592/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk592<F: Float>(t3529: F, t453: F, t1336: F, t140: F, t3532: F, t470: F, t5: F, t969: F, t22: F, t3118: F, t1210: F, t3725: F, t212: F, t23: F, t6: F, t161: F) -> (F, F, F, F, F, F, F) {
    let t5631 = t3529 * t453;
    let t5633 = t140 * t1336 * t5631;
    let t5634 = t470 * t3532;
    let t5680 = t5 * t969;
    let t5744 = t22 * t3118;
    let t5794 = t3725 * t1210;
    let t5814 = 1.0 / t23 / t212;
    let t5815 = t6 * t5814;
    let t5816 = t161 * t5815;
    (t5633, t5634, t5680, t5744, t5794, t5814, t5816)
}
