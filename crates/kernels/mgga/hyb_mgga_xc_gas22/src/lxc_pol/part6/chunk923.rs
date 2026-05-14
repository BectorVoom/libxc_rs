//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 923/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk923<F: Float>(t238: F, t242: F, t8697: F, t226: F, t8646: F, t1342: F, t2213: F) -> (F, F, F, F) {
    let t8699 = t238 * t242 * t8697;
    let t8701 = t226 * t8646;
    let t8703 = t238 * t242 * t8701;
    let t8706 = t238 * t2213 * t1342;
    (t8699, t8701, t8703, t8706)
}
