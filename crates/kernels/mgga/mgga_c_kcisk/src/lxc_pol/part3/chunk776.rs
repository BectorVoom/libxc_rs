//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 776/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk776<F: Float>(t12916: F, t12919: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12935: F, t12937: F, t12939: F, t12943: F, t12946: F, t12948: F, t12954: F, t12830: F, t3532: F) -> (F, F) {
    let t12956 = -0.28483875e1 * t12916 + 0.46074375e0 * t12919 - 0.33218518518518518518e0 * t12922 - 0.29896666666666666667e0 * t12927 - 0.39862222222222222223e0 * t12929 + 0.29896666666666666667e0 * t12931 + 0.19931111111111111111e0 * t12933 - 0.27385555555555555556e0 * t12935 + 0.16431333333333333333e0 * t12937 + 0.5477111111111111111e-1 * t12939 - 0.36514074074074074075e-1 * t12943 - 0.82156666666666666667e-1 * t12946 - 0.59793333333333333333e0 * t12948 + 0.11958666666666666667e1 * t12954;
    let t12957 = t3532 * t12830;
    (t12956, t12957)
}
