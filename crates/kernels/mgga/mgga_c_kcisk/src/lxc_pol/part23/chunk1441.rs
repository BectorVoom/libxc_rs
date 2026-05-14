//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1441/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1441<F: Float>(t113320: F, t113322: F, t113324: F, t113347: F, t113537: F, t113539: F, t113543: F, t113547: F, t113551: F, t113557: F, t113563: F, t113565: F, t113568: F, t113570: F, t113575: F, t114811: F, t114815: F, t114817: F, t114825: F, t114863: F, t115992: F, t240: F) -> (F,) {
    let t115996 = t113320 - t113322 + t113324 + t240 * (t113551 + t114825 + t114863 + t115992) + t113347 - t113537 + t113539 + t113543 - t113547 + t113557 + t113563 - t113565 - t113568 + t113570 + t113575 + t114811 + t114815 - t114817;
    (t115996,)
}
