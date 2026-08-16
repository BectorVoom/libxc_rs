//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2477/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2477(t48326: f64, t47149: f64, t3863: f64, t5569: f64, t3860: f64, t5571: f64, t9419: f64, t1882: f64, t4010: f64, t2682: f64, t4000: f64, t5677: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48327 = 24.0_f64 * t48326;
    let t48330 = 12.0_f64 * t47149;
    let t48331 = t3863 * t5569;
    let t48332 = 96.0_f64 * t48331;
    let t48333 = t3860 * t5569;
    let t48334 = 36.0_f64 * t48333;
    let t48335 = t5571 * t9419;
    let t48455 = t4010 * t1882;
    let t48486 = t820 * t4000 * t2682 * t5677;
    (t48327, t48330, t48332, t48334, t48335, t48455, t48486)
}
