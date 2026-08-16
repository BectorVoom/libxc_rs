//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1323/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1323<F: Float>(t1493: F, t5816: F, t77: F, t22656: F, t84: F, t1470: F, t21663: F, t1497: F, t5868: F, t4173: F, t5826: F, t5872: F) -> (F, F, F, F, F, F) {
    let t114260 = t77 * t1493 * t5816;
    let t114264 = t77 * t84 * t22656;
    let t114270 = t21663 * t1470;
    let t114288 = t77 * t5868 * t1497;
    let t114296 = t4173 * t5826;
    let t114301 = t77 * t1493 * t5872;
    (t114260, t114264, t114270, t114288, t114296, t114301)
}
