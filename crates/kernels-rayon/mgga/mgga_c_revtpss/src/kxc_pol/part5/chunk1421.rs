//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1421/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1421(t13254: f64, t13256: f64, t1456: f64, t1458: f64, t1464: f64, t18184: f64, t18186: f64, t18219: f64, t1914: f64, t1921: f64, t22533: f64, t22536: f64, t22542: f64, t22571: f64, t575: f64, t5790: f64, t5808: f64, t6937: f64, t6951: f64) -> f64 {
    let tv3rho32 = t1456 * t6951 + t1458 * t22571 + t1464 * t6937 + 2.0_f64 * t1914 * t5808 + 2.0_f64 * t1921 * t5790 + t22533 * t575 + t13254 + t13256 + t18184 + t18186 + t18219 + 2.0_f64 * t22536 + t22542;
    tv3rho32
}
