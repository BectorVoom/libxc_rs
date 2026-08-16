//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1224/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1224<F: Float>(t12810: F, t5352: F, t3720: F, t12269: F, t247: F, t3618: F, t12277: F, t1264: F, t12273: F, t1284: F, t3555: F, t3624: F) -> (F, F, F, F, F, F, F) {
    let t12811 = t12810 * t5352;
    let t12812 = t3720 * t12811;
    let t12816 = t247 * t3618 * t12269;
    let t12822 = t247 * t1264 * t12277;
    let t12828 = t247 * t1264 * t12273;
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    (t12811, t12812, t12816, t12822, t12828, t12831, t12832)
}
