//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1167/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1167(t1464: f64, t8900: f64, t1470: f64, t644: f64, t640: f64, t1493: f64, t36: f64, t606: f64, t37: f64, t1497: f64, t13426: f64, t8460: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t124438 = t8900 * t1464;
    let t125260 = t1470 * t644;
    let t125268 = t1470 * t640;
    let t125279 = t1493 * t36 * t606;
    let t125312 = t37 * t606;
    let t125335 = t1497 * t36;
    let t125336 = t125335 * t606;
    let t125384 = t13426 * t8460;
    (t124438, t125260, t125268, t125279, t125312, t125336, t125384)
}
