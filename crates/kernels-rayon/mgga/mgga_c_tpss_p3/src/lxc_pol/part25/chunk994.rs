//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 994/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk994(t3267: f64, t5410: f64, t12891: f64, t13677: f64, t13682: f64, t13687: f64, t13691: f64, t13695: f64, t13700: f64, t13703: f64, t13707: f64, t13711: f64, t13715: f64, t13719: f64, t3271: f64, t4413: f64) -> f64 {
    let t13722 = t3267 * t5410;
    let t13724 = t3271 * t13677 / 768.0_f64 - 5.0_f64 / 768.0_f64 * t3271 * t13682 + t3271 * t13687 / 768.0_f64 - t3271 * t13691 / 1536.0_f64 - t3271 * t13695 / 3072.0_f64 + t4413 * t13700 / 1536.0_f64 - 7.0_f64 / 576.0_f64 * t13703 - t12891 * t13707 / 512.0_f64 + t4413 * t13711 / 512.0_f64 - 5.0_f64 / 384.0_f64 * t3271 * t13715 + t3271 * t13719 / 384.0_f64 + 7.0_f64 / 4608.0_f64 * t13722;
    t13724
}
