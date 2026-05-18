//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1248/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1248<F: Float>(t2439: F, t7925: F, t94391: F, t94383: F, t25878: F, t98028: F, t94771: F, t97814: F, t27968: F, t3920: F, t25898: F, t98040: F) -> (F, F, F, F, F, F) {
    let t98311 = t7925 * t2439;
    let t98312 = t94391 * t98311;
    let t98314 = t94383 * t98311;
    let t98333 = t25878 * t98028;
    let t98338 = t94771 * t97814;
    let t98372 = t27968 * t3920;
    let t98380 = t98040 * t25898;
    (t98312, t98314, t98333, t98338, t98372, t98380)
}
