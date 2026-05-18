//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1080/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1080<F: Float>(t10308: F, t1466: F, t1925: F, t606: F, t7063: F, t860: F, t1444: F, t543: F, t1419: F, t116: F, t28159: F, t1892: F) -> (F, F, F, F, F, F, F) {
    let t60224 = t1466 * t10308;
    let t92669 = t606 * t1925;
    let t93341 = t7063 * t860;
    let t94396 = t543 * t1444;
    let t94801 = t7063 * t1419;
    let t97622 = t28159 * t116;
    let t98040 = t7063 * t1892;
    (t60224, t92669, t93341, t94396, t94801, t97622, t98040)
}
