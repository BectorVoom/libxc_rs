//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1749/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1749(t1459: f64, t1461: f64, t2113: f64, t2115: f64, t26716: f64, t26730: f64, t26734: f64, t26737: f64, t26740: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, t7547: f64, t7554: f64, t7557: f64) -> f64 {
    let t26743 = 12.0_f64 * t1459 * t7554 + 6.0_f64 * t1459 * t7557 + 6.0_f64 * t1461 * t7547 + 6.0_f64 * t2113 * t4162 + 3.0_f64 * t2113 * t4165 + 3.0_f64 * t2115 * t4158 + t26716 * t573 + 6.0_f64 * t26730 * t572 + 12.0_f64 * t26734 * t572 + 6.0_f64 * t26737 * t572 + 3.0_f64 * t26740 * t572;
    t26743
}
