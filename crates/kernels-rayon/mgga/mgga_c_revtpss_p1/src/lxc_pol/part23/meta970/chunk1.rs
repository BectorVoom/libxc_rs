//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3271/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3271(t1353: f64, t13768: f64, t13902: f64, t13910: f64, t1392: f64, t1394: f64, t1412: f64, t1877: f64, t21969: f64, t22236: f64, t22249: f64, t22287: f64, t22809: f64, t22813: f64, t22944: f64, t22947: f64, t22950: f64, t539: f64, t5591: f64, t5650: f64, t5651: f64, t6816: f64, t85442: f64, t9940: f64) -> f64 {
    let t86052 = -12.0_f64 * t1353 * t1412 * t22809 * t5650 - 360.0_f64 * t1353 * t22813 * t5650 * t9940 + 180.0_f64 * t13768 * t22287 * t5650 - 36.0_f64 * t13910 * t5650 * t6816 + 3.0_f64 * t1394 * t539 * t85442 - 36.0_f64 * t21969 * t5650 * t5651 + 180.0_f64 * t22236 * t5591 * t5650 - 36.0_f64 * t13902 * t22947 + 60.0_f64 * t1392 * t22944 + 3.0_f64 * t1392 * t22950 + 9.0_f64 * t1877 * t22249;
    t86052
}
