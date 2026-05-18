//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1155/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1155<F: Float>(t19: F, t2986: F, t669: F, t1815: F, t1862: F, t547: F, t5878: F, t1056: F, t3: F, t1823: F, t1816: F, t1867: F) -> (F, F, F, F, F, F) {
    let t19574 = t19 * t2986 * t669;
    let t19577 = t19 * t1815 * t1862;
    let t19579 = t547 * t5878;
    let t19643 = t3 * t1056;
    let t19664 = t19 * t1815 * t1823;
    let t19698 = t1867 * t1816;
    (t19574, t19577, t19579, t19643, t19664, t19698)
}
