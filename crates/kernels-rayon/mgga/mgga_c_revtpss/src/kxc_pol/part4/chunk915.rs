//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 915/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk915(t225: f64, t5638: f64, t5642: f64, t539: f64, t73: f64, t1412: f64, t1868: f64, t1353: f64, t1394: f64, t5591: f64, t1392: f64, t1395: f64, t1877: f64, t1879: f64, t541: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5644 = (t5638 + t5642) * t225;
    let t5650 = t539 * t73;
    let t5651 = t1412 * t1868;
    let t5652 = t5651 * t1353;
    let t5655 = t1394 * t5591;
    let t5658 = 3.0_f64 * t1392 * t1879 + 3.0_f64 * t1395 * t1877 + 3.0_f64 * t539 * t5655 - t541 * t5644 - 12.0_f64 * t5650 * t5652;
    (t5644, t5650, t5651, t5652, t5655, t5658)
}
