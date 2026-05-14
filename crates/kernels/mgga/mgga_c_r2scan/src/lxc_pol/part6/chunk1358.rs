//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1358/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1358<F: Float>(t2531: F, t6212: F, t6209: F, t6211: F, t2682: F, t6422: F, t20825: F, t2609: F, t2252: F, t2562: F, t2148: F, t7628: F, t2097: F, t2665: F, t546: F, t6477: F) -> (F, F, F, F, F, F) {
    let t25737 = t6212 * t2531;
    let t25739 = t6209 * t6211 * t25737;
    let t25740 = 0.19043987679069580388e-1 * t25739;
    let t25742 = t6422 * t2682;
    let t25744 = t20825 * t2609;
    let t25746 = t2562 * t2252;
    let t25748 = t7628 * t2148 * t25746;
    let t25751 = t2665 * t2097;
    let t25752 = t546 * t25751;
    let t25753 = t25752 * t6477;
    (t25740, t25742, t25744, t25748, t25751, t25753)
}
