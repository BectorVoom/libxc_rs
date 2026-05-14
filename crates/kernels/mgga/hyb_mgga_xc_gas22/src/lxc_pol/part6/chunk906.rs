//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 906/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk906<F: Float>(t2028: F, t3160: F, t1240: F, t136: F, t2024: F, t2027: F, t2154: F, t2949: F, t2986: F, t3124: F, t684: F, t687: F, t7831: F, t8224: F, t8441: F, t8446: F, t8450: F, t8452: F, t8454: F, t8457: F, t8462: F, t8465: F) -> (F, F) {
    let t8469 = t3160 * t2028;
    let t8473 = t8224 / 96.0 - 3.0 / 64.0 * t136 * t8441 - 3.0 / 64.0 * t1240 * t2154 + 3.0 / 16.0 * t2949 * t8446 - t8450 - t8452 + t8454 / 96.0 + t684 * t2986 * t8457 / 32.0 - t8462 + 3.0 / 32.0 * t7831 * t3124 - t684 * t687 * t8465 / 64.0 - t2024 * t2027 * t8469 / 48.0;
    (t8469, t8473)
}
