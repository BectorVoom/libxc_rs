//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2057/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2057(t7286: f64, t98308: f64, t2439: f64, t7925: f64, t94391: f64, t94383: f64, t25878: f64, t98028: f64, t94771: f64, t97814: f64, t1903: f64, t25931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98310 = 0.14456046980341999104e-1_f64 * t98308 * t7286;
    let t98311 = t7925 * t2439;
    let t98312 = t94391 * t98311;
    let t98314 = t94383 * t98311;
    let t98333 = t25878 * t98028;
    let t98338 = t94771 * t97814;
    let t98340 = t25931 * t1903;
    (t98310, t98312, t98314, t98333, t98338, t98340)
}
