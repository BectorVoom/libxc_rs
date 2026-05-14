//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1215/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1215<F: Float>(t25737: F, t6209: F, t6211: F, t2682: F, t6422: F, t2097: F, t2665: F, t546: F, t6477: F, t565: F, t6482: F, t20137: F, t6475: F, t7257: F, t6480: F, t7261: F) -> (F, F, F, F, F, F, F, F) {
    let t25739 = t6209 * t6211 * t25737;
    let t25740 = 0.19043987679069580388e-1 * t25739;
    let t25742 = t6422 * t2682;
    let t25751 = t2665 * t2097;
    let t25752 = t546 * t25751;
    let t25753 = t25752 * t6477;
    let t25754 = 0.19043987679069580388e-1 * t25753;
    let t25755 = t565 * t25751;
    let t25756 = t25755 * t6482;
    let t25759 = t6475 * t20137 * t7257;
    let t25764 = t6480 * t20137 * t7261;
    (t25740, t25742, t25752, t25754, t25755, t25756, t25759, t25764)
}
