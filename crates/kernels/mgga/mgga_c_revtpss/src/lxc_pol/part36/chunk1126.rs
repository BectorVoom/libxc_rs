//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1126/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1126<F: Float>(t2471: F, t27202: F, t15003: F, t93194: F, t7759: F, t822: F, t2470: F, t27340: F, t25387: F, t136: F, t2457: F, t7778: F, t25299: F, t1568: F, t786: F, t25410: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99307 = t27202 * t2471;
    let t99313 = t93194 * t15003;
    let t99334 = t822 * t7759;
    let t99365 = t27340 * t2470;
    let t99366 = t25387 * t99365;
    let t99380 = t7778 * t136 * t2457;
    let t99381 = t25299 * t99380;
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    (t99307, t99313, t99334, t99365, t99366, t99380, t99381, t99403, t99404)
}
