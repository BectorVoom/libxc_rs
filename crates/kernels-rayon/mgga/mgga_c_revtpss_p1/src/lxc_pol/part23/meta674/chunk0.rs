//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2410/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2410(t11627: f64, t42859: f64, t342: f64, t12077: f64, t989: f64, t12153: f64, t3057: f64, t1071: f64, t11200: f64, t3494: f64, t3519: f64, t13026: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43536 = t42859 * t11627;
    let t43537 = t342 * t43536;
    let t43574 = t989 * t12077;
    let t43598 = t3057 * t12153;
    let t43637 = t11200 * t1071;
    let t43752 = 1.0_f64 / t3519 / t3494;
    let t43764 = t240 * t13026;
    (t43536, t43537, t43574, t43598, t43637, t43752, t43764)
}
