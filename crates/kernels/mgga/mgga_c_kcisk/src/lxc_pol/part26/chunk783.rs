//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 783/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk783<F: Float>(t2737: F, t2740: F, t9516: F, t9534: F, t9536: F, t9549: F, t9816: F, t9819: F, t9822: F, t9825: F, t9851: F, t9855: F, t9860: F, t9864: F, t9869: F, t2347: F, t2748: F) -> (F, F) {
    let t9878 = -0.52083333333333333333e-2 * t9851 * t2740 + 0.20104166666666666667e-2 * t9516 * t9855 - 0.52083333333333333333e-2 * t9860 * t2740 - t9534 - 0.17361111111111111111e-2 * t9536 * t9864 + 0.52083333333333333333e-2 * t2737 * t9869 + 0.52083333333333333333e-2 * t2737 * t9855 + t9549 + 0.11607361111111111111e-2 * t9816 + 0.17411041666666666666e-2 * t9819 - 0.17411041666666666666e-2 * t9822 + 0.11607361111111111111e-2 * t9825;
    let t9882 = t2748 * t2347;
    (t9878, t9882)
}
