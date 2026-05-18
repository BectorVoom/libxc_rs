//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1239/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1239<F: Float>(t3418: F, t6669: F, t2314: F, t8709: F, t3385: F, t6712: F, t3352: F, t6564: F, t1370: F, t6641: F, t6667: F, t1346: F, t6579: F) -> (F, F, F, F, F, F, F) {
    let t24989 = t3418 * t6669;
    let t24996 = t8709 * t2314;
    let t25049 = t3385 * t6712;
    let t25116 = t3352 * t6564;
    let t25129 = t6641 * t1370;
    let t25132 = t6667 * t1370;
    let t25146 = t6579 * t1346;
    (t24989, t24996, t25049, t25116, t25129, t25132, t25146)
}
