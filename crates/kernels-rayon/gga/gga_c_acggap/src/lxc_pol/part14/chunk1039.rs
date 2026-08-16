//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1039/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1039(t2333: f64, t848: f64, t2342: f64, t30005: f64, t2131: f64, t2132: f64, t2331: f64, t847: f64, t7994: f64, t8998: f64, t32041: f64, t36019: f64, t7932: f64) -> (f64, f64, f64, f64, f64) {
    let t36531 = t848 * t2333;
    let t36533 = t30005 * t2342;
    let t36541 = t2131 * t2132 * t2331 * t847;
    let t36543 = t8998 * t7994;
    let t36555 = t32041 * t7932 * t36019;
    (t36531, t36533, t36541, t36543, t36555)
}
