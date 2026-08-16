//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1013/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1013<F: Float>(t11695: F, t3225: F, t773: F, t826: F, t10264: F, t3212: F, t3724: F, t3209: F, t3765: F, t7553: F, t3679: F, t7557: F) -> (F, F, F, F, F, F, F) {
    let t11696 = t3225 * t11695;
    let t11698 = t826 * t773;
    let t11699 = t10264 * t11698;
    let t11701 = t3212 * t3724;
    let t11703 = t3209 * t3724;
    let t11728 = t7553 * t3765;
    let t11730 = t3679 * t7557;
    (t11696, t11698, t11699, t11701, t11703, t11728, t11730)
}
