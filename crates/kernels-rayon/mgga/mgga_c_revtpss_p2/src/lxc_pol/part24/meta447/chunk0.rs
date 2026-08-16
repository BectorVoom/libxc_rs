//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1408/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1408(t13665: f64, t9863: f64, t9866: f64, t9575: f64, t9572: f64, t3863: f64, t5569: f64, t3860: f64, t5571: f64, t9419: f64, t1882: f64, t4010: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48304 = t13665 * t9863;
    let t48306 = t13665 * t9866;
    let t48313 = t13665 * t9575;
    let t48324 = t13665 * t9572;
    let t48331 = t3863 * t5569;
    let t48333 = t3860 * t5569;
    let t48335 = t5571 * t9419;
    let t48455 = t4010 * t1882;
    (t48304, t48306, t48313, t48324, t48331, t48333, t48335, t48455)
}
