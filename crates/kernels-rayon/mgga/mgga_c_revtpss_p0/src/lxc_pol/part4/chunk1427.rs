//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1427/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1427(t13254: f64, t13256: f64, t1456: f64, t1458: f64, t1464: f64, t18178: f64, t18184: f64, t18186: f64, t18217: f64, t1914: f64, t1921: f64, t4154: f64, t4168: f64, t575: f64, t5790: f64, t5808: f64, t9263: f64, t9265: f64, t9267: f64) -> f64 {
    let tv3rho31 = 2.0_f64 * t1456 * t5808 + t1458 * t18217 + 2.0_f64 * t1464 * t5790 + t18178 * t575 + t1914 * t4168 + t1921 * t4154 + t13254 + t13256 + t18184 + t18186 + t9263 + 2.0_f64 * t9265 + t9267;
    tv3rho31
}
