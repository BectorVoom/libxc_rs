//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1580/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1580<F: Float>(t2661: F, t3992: F, t543: F, t86205: F, t221: F, t22912: F, t4018: F, t4019: F, t6869: F, t73920: F, t1883: F, t22245: F) -> (F, F, F, F) {
    let t86244 = t2661 * t3992 * t86205 * t543;
    let t86256 = t4018 * t4019 * t221 * t22912;
    let t86260 = t2661 * t3992 * t73920 * t6869;
    let t86264 = t2661 * t3992 * t22245 * t1883;
    (t86244, t86256, t86260, t86264)
}
