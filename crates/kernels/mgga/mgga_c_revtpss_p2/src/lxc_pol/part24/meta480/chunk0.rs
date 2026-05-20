//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1469/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1469<F: Float>(t3451: F, t6481: F, t12555: F, t6534: F, t3565: F, t6563: F, t225: F, t1261: F, t12879: F, t247: F, t6429: F, t11262: F, t1247: F, t6624: F) -> (F, F, F, F, F, F) {
    let t69488 = t6481 * t3451;
    let t69511 = t6534 * t12555;
    let t69636 = t6563 * t3565;
    let t69637 = t69636 * t225;
    let t69661 = t1261 * t247 * t12879 * t6429;
    let t69668 = t1247 * t11262 * t6624;
    (t69488, t69511, t69636, t69637, t69661, t69668)
}
