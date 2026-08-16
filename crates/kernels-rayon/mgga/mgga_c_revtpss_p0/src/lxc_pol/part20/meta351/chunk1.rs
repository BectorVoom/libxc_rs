//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1280/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1280(t13225: f64, t575: f64, t1464: f64, t4153: f64, t1455: f64, t4168: f64, t13250: f64, t571: f64, t2565: f64, t702: f64, t9305: f64) -> (f64, f64, f64, f64, f64) {
    let t39397 = t13225 * t575;
    let t39399 = t4153 * t1464;
    let t39401 = t1455 * t4168;
    let t39403 = t571 * t13250;
    let t39419 = 8.0_f64 * t2565 * t702 * t9305;
    (t39397, t39399, t39401, t39403, t39419)
}
