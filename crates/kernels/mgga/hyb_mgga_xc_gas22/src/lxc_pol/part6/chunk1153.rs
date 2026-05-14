//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1153/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1153<F: Float>(t1270: F, t6359: F, t180: F, t8354: F, t2004: F, t3279: F, t136: F, t550: F, t8440: F, t1318: F, t2151: F, t2014: F, t684: F, t8562: F, t3146: F, t6469: F) -> (F, F, F, F, F, F, F) {
    let t24320 = t6359 * t1270;
    let t24354 = t180 * t8354;
    let t24426 = t2004 * t3279;
    let t24439 = t136 * t550 * t8440;
    let t24455 = t2151 * t1318;
    let t24461 = t684 * t2014 * t8562;
    let t24464 = t684 * t6469 * t3146;
    (t24320, t24354, t24426, t24439, t24455, t24461, t24464)
}
