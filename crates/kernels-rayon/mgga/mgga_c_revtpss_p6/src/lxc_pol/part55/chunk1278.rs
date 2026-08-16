//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1278/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1278(t33281: f64, t34402: f64, t34410: f64, t1497: f64, t32798: f64, t33280: f64, t8621: f64, t124217: f64, t1493: f64, t8737: f64, t68: f64, t124235: f64, t1469: f64, t32802: f64, t8442: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t130848 = t34402 * t33281;
    let t130858 = t34410 * t33281;
    let t130862 = t32798 * t8621 * t33280 * t1497;
    let t130866 = t8737 * t8621 * t124217 * t1493;
    let t130882 = t68 * t1497;
    let t130893 = t32802 * t8442 * t124235 * t1469;
    (t130848, t130858, t130862, t130866, t130882, t130893)
}
