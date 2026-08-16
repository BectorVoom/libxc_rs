//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2186/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2186<F: Float>(t27932: F, t74477: F, t74419: F, t98196: F, t74423: F, t22021: F, t25986: F, t2661: F, t22068: F, t25972: F, t25978: F, t6880: F) -> (F, F, F, F, F, F) {
    let t108615 = t27932 * t74477;
    let t108617 = t98196 * t74419;
    let t108619 = t27932 * t74423;
    let t108623 = t2661 * t25986 * t22021;
    let t108625 = t25972 * t22068;
    let t108627 = t25978 * t6880;
    (t108615, t108617, t108619, t108623, t108625, t108627)
}
