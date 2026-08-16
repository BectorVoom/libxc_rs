//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 972/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk972(t1416: f64, t9779: f64, t124: f64, t212: f64, t2237: f64, t800: f64, t1376: f64, t123: f64, t125: f64, t2452: f64, t9720: f64, t235: f64, t4086: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9780 = t9779 * t1416;
    let t9784 = t800 * t124 * t2237 * t212;
    let t9786 = 0.72250660161932334527e-3_f64 * t9784 * t1376;
    let t9789 = t123 * t125 * t9720 * t2452;
    let t9791 = 0.11294745624363664198e-6_f64 * t9789 * t1376;
    let t9792 = t4086 * t235;
    (t9780, t9784, t9786, t9789, t9791, t9792)
}
