//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1173/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1173<F: Float>(t2014: F, t2034: F, t86825: F, t1843: F, t30004: F, t651: F, t25082: F, t30122: F, t33651: F, t18245: F, t7742: F, t114378: F, t1937: F, t30138: F, t7735: F, t29576: F, t7898: F) -> (F, F, F, F, F, F, F) {
    let t114407 = t2014 * t2034 * t86825;
    let t114410 = 6.0 * t651 * t1843 * t30004;
    let t114415 = 18.0 * t25082 * t33651 * t30122;
    let t114417 = 6.0 * t18245 * t7742;
    let t114419 = 6.0 * t114378 * t1937;
    let t114421 = 12.0 * t30138 * t7735;
    let t114427 = 6.0 * t7898 * t29576;
    (t114407, t114410, t114415, t114417, t114419, t114421, t114427)
}
