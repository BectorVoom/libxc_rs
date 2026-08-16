//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1377/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1377(t111419: f64, t113019: f64, t113022: f64, t113025: f64, t113053: f64, t113054: f64, t116867: f64, t116876: f64, t1458: f64, t1914: f64, t1921: f64, t2168: f64, t2172: f64, t25049: f64, t25072: f64, t3: f64, t30975: f64, t30993: f64, t575: f64, t6937: f64, t6951: f64, t8241: f64, t8249: f64) -> f64 {
    let tv4rho3sigma11 = t116867 * t3 * t575 + t116876 * t1458 + 3.0_f64 * t1914 * t30993 + 3.0_f64 * t1921 * t30975 + t2168 * t25072 + t2172 * t25049 + 3.0_f64 * t6937 * t8249 + 3.0_f64 * t6951 * t8241 + 3.0_f64 * t111419 + 3.0_f64 * t113019 + 6.0_f64 * t113022 + 3.0_f64 * t113025 + 3.0_f64 * t113053 + 6.0_f64 * t113054;
    tv4rho3sigma11
}
