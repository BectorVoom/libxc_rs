//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1849/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1849(t1458: f64, t1914: f64, t1921: f64, t25049: f64, t25072: f64, t3: f64, t575: f64, t6937: f64, t6951: f64, t75808: f64, t86897: f64, t86903: f64, t86909: f64, t92517: f64, t92552: f64) -> f64 {
    let tv4rho44 = t3 * t575 * t92517 + t1458 * t92552 + 4.0_f64 * t1914 * t25072 + 4.0_f64 * t1921 * t25049 + 6.0_f64 * t6937 * t6951 + 4.0_f64 * t75808 + 12.0_f64 * t86897 + 12.0_f64 * t86903 + 4.0_f64 * t86909;
    tv4rho44
}
