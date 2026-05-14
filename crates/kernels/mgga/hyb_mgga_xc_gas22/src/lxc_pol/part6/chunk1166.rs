//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1166/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1166<F: Float>(t2559: F, t3524: F, t1414: F, t7058: F, t2576: F, t3557: F, t1426: F, t7108: F, t9031: F, t997: F, t9039: F, t978: F, t2537: F, t7001: F, t2598: F, t6992: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25556 = t3524 * t2559;
    let t25561 = t1414 * t7058;
    let t25624 = t3557 * t2576;
    let t25627 = t1426 * t7108;
    let t25630 = t9031 * t997;
    let t25633 = t9039 * t978;
    let t25643 = t3524 * t2537;
    let t25648 = t1414 * t7001;
    let t25651 = t3557 * t2598;
    let t25654 = t1426 * t6992;
    (t25556, t25561, t25624, t25627, t25630, t25633, t25643, t25648, t25651, t25654)
}
