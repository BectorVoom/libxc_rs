//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1217/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1217<F: Float>(t2970: F, t7848: F, t7861: F, t240: F, t6184: F, t92: F, t7843: F, t7866: F, t639: F, t7867: F, t1804: F, t6214: F, t8211: F) -> (F, F, F, F, F) {
    let t23696 = t2970 * t7848 * t7861;
    let t23699 = t240 * t6184 * t92;
    let t23701 = t7866 * t23699 * t7843;
    let t23706 = t7867 * t639;
    let t23726 = t1804 * t6214 * t8211;
    (t23696, t23699, t23701, t23706, t23726)
}
