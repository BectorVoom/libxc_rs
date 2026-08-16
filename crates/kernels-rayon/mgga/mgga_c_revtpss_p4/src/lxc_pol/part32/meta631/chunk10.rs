//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2053/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2053(t2118: f64, t6936: f64, t104062: f64, t111304: f64, t111345: f64, t111390: f64, t1456: f64, t1458: f64, t1464: f64, t1914: f64, t1921: f64, t2111: f64, t22533: f64, t22571: f64, t28945: f64, t28993: f64, t3: f64, t30627: f64, t30663: f64, t575: f64, t5790: f64, t5808: f64, t6937: f64, t7560: f64, t8114: f64, t8130: f64) -> f64 {
    let t111405 = t6936 * t2118;
    let t111407 = t3 * t111304 * t575 + t1458 * (t111345 + t111390) + t1456 * t30663 + t22533 * t2118 + 2.0_f64 * t8114 * t5808 + t30627 * t1464 + 2.0_f64 * t5790 * t8130 + 2.0_f64 * t28945 * t1921 + 2.0_f64 * t1914 * t28993 + t2111 * t22571 + t104062 + t111405 + t6937 * t7560;
    t111407
}
