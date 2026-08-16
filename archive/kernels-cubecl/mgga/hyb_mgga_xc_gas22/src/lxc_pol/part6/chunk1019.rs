//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1019/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1019<F: Float>(t2867: F, t3756: F, t532: F, t2824: F, t3705: F, t3687: F, t531: F, t1143: F, t3697: F, t1159: F, t7636: F, t524: F) -> (F, F, F, F, F, F, F, F) {
    let t9538 = t2867 * t3756;
    let t9542 = t2867 * t532;
    let t9545 = t3705 * t2824;
    let t9548 = t3687 * t531;
    let t9549 = t1143 * t9548;
    let t9552 = t3697 * t2824;
    let t9557 = t7636 * t1159;
    let t9558 = t524 * t9557;
    (t9538, t9542, t9545, t9548, t9549, t9552, t9557, t9558)
}
