//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2130/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2130<F: Float>(t25207: F, t98651: F, t1468: F, t2411: F, t14365: F, t1544: F, t2257: F, t198: F, t205: F, t7086: F, t4433: F, t890: F) -> (F, F, F, F, F) {
    let t98652 = t25207 * t98651;
    let t98658 = t2411 * t1468;
    let t98659 = t98658 * t14365;
    let t98662 = t2257 * t1544;
    let t98669 = t198 * t205 * t7086;
    let t98674 = t4433 * t890;
    (t98652, t98659, t98662, t98669, t98674)
}
