//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1088/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1088(t475: f64, t5389: f64, t467: f64, t1264: f64, t5056: f64, t247: f64, t3629: f64, t5351: f64, t3626: f64, t3627: f64, t471: f64, t1715: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
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
