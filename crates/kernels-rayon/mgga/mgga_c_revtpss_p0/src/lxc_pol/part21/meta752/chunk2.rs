//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2632/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2632(t48396: f64, t48419: f64, t4010: f64, t5591: f64, t1353: f64, t13716: f64, t13892: f64, t13902: f64, t13910: f64, t13911: f64, t13914: f64, t13917: f64, t1392: f64, t1394: f64, t1395: f64, t1412: f64, t1879: f64, t3829: f64, t3889: f64, t4050: f64, t539: f64, t5644: f64, t5650: f64, t5651: f64, t9628: f64, t9872: f64) -> (f64, f64, f64) {
    let t48421 = t48396 / 2.0_f64 + t48419 / 2.0_f64;
    let t48432 = t4010 * t5591;
    let t48436 = -36.0_f64 * t1353 * t13716 * t1412 * t5650 - 36.0_f64 * t13910 * t3889 * t5650 + 3.0_f64 * t1394 * t48421 * t539 + 180.0_f64 * t3829 * t48432 * t5650 - 12.0_f64 * t5650 * t5651 * t9628 + 9.0_f64 * t13892 * t1395 - 72.0_f64 * t13902 * t13911 - 36.0_f64 * t13902 * t13914 + 9.0_f64 * t13917 * t1392 + 3.0_f64 * t1879 * t9872 - 36.0_f64 * t4050 * t5644;
    (t48421, t48432, t48436)
}
