//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 998/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk998<F: Float>(t2594: F, t3596: F, t3604: F, t7165: F, t2598: F, t3579: F, t3605: F, t260: F, t3557: F, t1006: F, t9195: F, t997: F) -> (F, F, F, F, F) {
    let t9282 = t3596 * t2594;
    let t9285 = t3604 * t7165;
    let t9288 = t2598 * t3579;
    let t9289 = t9288 * t3605;
    let t9296 = t260 * t3557;
    let t9306 = t997 * t9195 * t1006;
    (t9282, t9285, t9289, t9296, t9306)
}
