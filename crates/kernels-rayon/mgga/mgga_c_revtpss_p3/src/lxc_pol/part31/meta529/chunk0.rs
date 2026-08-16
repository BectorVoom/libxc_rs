//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1905/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1905(t28076: f64, t72: f64, t1927: f64, t6977: f64, t7715: f64, t6973: f64, t7719: f64, t4237: f64, t76: f64, t1926: f64, t13269: f64, t38: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28077 = t28076 * t72;
    let t28078 = t28077 * t1927;
    let t28081 = t7715 * t6977;
    let t28086 = t6973 * t7719;
    let t28089 = t76 * t4237;
    let t28090 = t1926 * t28089;
    let t28093 = t13269 * t38;
    (t28077, t28078, t28081, t28086, t28089, t28090, t28093)
}
