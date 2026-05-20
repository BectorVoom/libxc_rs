//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2832/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2832<F: Float>(t11263: F, t3169: F, t3043: F, t3140: F, t3149: F, t3160: F, t11874: F, t16048: F, t12046: F, t15905: F, t994: F, t3114: F, t42416: F) -> (F, F, F, F, F, F) {
    let t42656 = t3169 * t11263;
    let t42664 = t3043 * t3140;
    let t42665 = t42664 * t3149;
    let t42672 = t42664 * t3160;
    let t42675 = t11874 * t16048;
    let t42690 = t994 * t12046 * t15905;
    let t42695 = t3114 * t42416;
    (t42656, t42665, t42672, t42675, t42690, t42695)
}
