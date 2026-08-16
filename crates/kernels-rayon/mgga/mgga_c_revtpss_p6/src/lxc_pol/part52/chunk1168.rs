//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1168/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1168(t1426: f64, t1882: f64, t121011: f64, t247: f64, t94396: f64, t125627: f64, t1399: f64, t1892: f64, t31805: f64, t1381: f64, t8590: f64, t32195: f64, t32206: f64, t3936: f64, t5591: f64) -> (f64, f64, f64, f64, f64) {
    let t125639 = t1426 * t1882;
    let t125642 = t121011 * t247 * t125639 * t94396;
    let t125646 = t121011 * t247 * t125627 * t1399;
    let t125648 = t31805 * t1892;
    let t125650 = t125648 * t8590 * t1381;
    let t125659 = t32206 * t3936 * t32195 * t5591;
    (t125642, t125646, t125648, t125650, t125659)
}
