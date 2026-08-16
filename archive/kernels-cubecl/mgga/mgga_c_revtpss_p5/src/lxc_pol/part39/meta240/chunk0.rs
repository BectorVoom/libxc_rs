//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 915/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk915<F: Float>(t475: F, t5389: F, t467: F, t1264: F, t5056: F, t247: F, t3629: F, t5351: F, t3626: F, t3627: F, t471: F, t1715: F) -> (F, F, F, F, F, F, F) {
    let t5390 = t475 * t5389;
    let t5391 = t467 * t5390;
    let t5396 = t1264 * t5056;
    let t5397 = t247 * t5396;
    let t5401 = t5351 * t3629;
    let t5402 = t3626 * t5401;
    let t5405 = t3627 * t471;
    let t5406 = t1715 * t5405;
    (t5390, t5391, t5397, t5401, t5402, t5405, t5406)
}
