//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1085/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1085(t1214: f64, t471: f64, t5351: f64, t3720: f64, t140: f64, t1781: f64, t1222: f64, t127: f64, t1789: f64, t371: f64, t1235: f64, t1219: f64, t1778: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5352 = t471 * t1214;
    let t5353 = t5351 * t5352;
    let t5354 = t3720 * t5353;
    let t5357 = t140 * t1781;
    let t5358 = t1222 * t5357;
    let t5362 = t371 * t127 * t1789;
    let t5363 = t1235 * t5362;
    let t5366 = t1778 * t1219;
    (t5352, t5353, t5354, t5358, t5362, t5363, t5366)
}
