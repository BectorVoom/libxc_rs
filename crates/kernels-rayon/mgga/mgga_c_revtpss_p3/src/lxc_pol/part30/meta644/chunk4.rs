//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2265/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2265(t101613: f64, t101617: f64, t101619: f64, t101621: f64, t101625: f64, t101628: f64, t101632: f64, t101634: f64, t101640: f64, t101642: f64, t101645: f64, t101648: f64, t1461: f64, t18208: f64, t18214: f64, t1918: f64, t2170: f64, t27102: f64, t29480: f64, t4162: f64, t5802: f64, t7696: f64, t8245: f64) -> f64 {
    let t105775 = 6.0_f64 * t1461 * t29480 + 12.0_f64 * t18208 * t2170 + 3.0_f64 * t18214 * t2170 + 3.0_f64 * t1918 * t27102 + 6.0_f64 * t4162 * t8245 + 12.0_f64 * t5802 * t7696 + t101613 + t101617 + t101619 + t101621 + t101625 + t101628 + t101632 + t101634 + t101640 + t101642 + t101645 + t101648;
    t105775
}
