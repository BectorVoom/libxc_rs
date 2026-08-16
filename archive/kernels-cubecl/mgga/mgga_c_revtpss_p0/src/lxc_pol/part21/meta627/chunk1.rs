//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2391/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2391<F: Float>(t10868: F, t240: F, t10871: F, t2661: F, t40479: F, t10726: F, t2723: F, t10638: F, t231: F, t243: F, t2662: F, t10722: F, t2656: F) -> (F, F, F, F, F) {
    let t40693 = t10868 * t240;
    let t40696 = t2661 * t40693 * t40479 * t10871;
    let t40700 = t2661 * t10726 * t40479 * t2723;
    let t40705 = t2661 * t2662 * t243 * t10638 * t231;
    let t40707 = t10722 * t2656;
    (t40693, t40696, t40700, t40705, t40707)
}
