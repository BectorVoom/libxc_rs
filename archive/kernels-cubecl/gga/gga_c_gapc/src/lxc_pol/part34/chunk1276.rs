//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1276/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1276<F: Float>(t11387: F, t21053: F, t21054: F, t11365: F, t5285: F, t5703: F, t1386: F, t3663: F, t3665: F, t2981: F, t34754: F, t458: F) -> (F, F, F, F) {
    let t35231 = t21053 * t11387 * t21054;
    let t35234 = t5285 * t11365 * t5703;
    let t35240 = t1386 * t3663 * t3665;
    let t35243 = t34754 * t2981 * t458;
    (t35231, t35234, t35240, t35243)
}
