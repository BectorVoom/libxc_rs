//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1216/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1216(t17944: f64, t17971: f64, t17947: f64, t17950: f64, t17952: f64, t17957: f64, t17961: f64, t17965: f64, t17967: f64, t17969: f64, t17975: f64, t17977: f64, t17979: f64) -> (f64, f64, f64) {
    let t18737 = 35.0_f64 / 216.0_f64 * t17944;
    let t18746 = 119.0_f64 / 3456.0_f64 * t17971;
    let t18750 = t18737 + 7.0_f64 / 36.0_f64 * t17947 + t17950 / 8.0_f64 - t17952 / 24.0_f64 + t17957 / 384.0_f64 + 7.0_f64 / 576.0_f64 * t17961 + t17965 / 96.0_f64 - t17967 / 768.0_f64 - t17969 / 768.0_f64 + t18746 + 7.0_f64 / 144.0_f64 * t17975 + 5.0_f64 / 192.0_f64 * t17977 - t17979 / 192.0_f64;
    (t18737, t18746, t18750)
}
