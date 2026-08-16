//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2266/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2266(t104094: f64, t105759: f64, t105762: f64, t105775: f64, t1456: f64, t1458: f64, t1464: f64, t18178: f64, t1914: f64, t1921: f64, t2172: f64, t27090: f64, t27110: f64, t29469: f64, t29490: f64, t3: f64, t4154: f64, t4168: f64, t575: f64, t5790: f64, t5808: f64, t7691: f64, t7700: f64, t8241: f64, t8249: f64, t96690: f64) -> f64 {
    let t105789 = t18178 * t2172 + 2.0_f64 * t96690 + t1914 * t27110 + 2.0_f64 * t5790 * t7700 + t104094 + t1458 * (t105762 + t105775) + 2.0_f64 * t1456 * t29490 + t4154 * t8249 + t3 * t105759 * t575 + t27090 * t1921 + 2.0_f64 * t7691 * t5808 + 2.0_f64 * t29469 * t1464 + t8241 * t4168;
    t105789
}
