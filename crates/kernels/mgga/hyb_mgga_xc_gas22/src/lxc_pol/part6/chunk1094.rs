//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1094/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1094<F: Float>(t547: F, t5878: F, t1056: F, t3: F, t1815: F, t1823: F, t19: F, t1816: F, t1867: F, t5885: F, t1801: F, t1896: F, t3023: F, t580: F, t1909: F, t6012: F) -> (F, F, F, F, F, F, F, F) {
    let t19579 = t547 * t5878;
    let t19643 = t3 * t1056;
    let t19664 = t19 * t1815 * t1823;
    let t19698 = t1867 * t1816;
    let t19700 = t547 * t5885;
    let t19706 = 1.0 / t1896 / t1801;
    let t19735 = t3023 * t580;
    let t19737 = t6012 * t1909;
    (t19579, t19643, t19664, t19698, t19700, t19706, t19735, t19737)
}
