//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1580/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1580(t2661: f64, t3992: f64, t543: f64, t86205: f64, t221: f64, t22912: f64, t4018: f64, t4019: f64, t6869: f64, t73920: f64, t1883: f64, t22245: f64) -> (f64, f64, f64, f64) {
    let t86244 = t2661 * t3992 * t86205 * t543;
    let t86256 = t4018 * t4019 * t221 * t22912;
    let t86260 = t2661 * t3992 * t73920 * t6869;
    let t86264 = t2661 * t3992 * t22245 * t1883;
    (t86244, t86256, t86260, t86264)
}
