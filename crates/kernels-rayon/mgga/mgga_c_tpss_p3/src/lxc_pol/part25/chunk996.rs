//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 996/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk996(t1233: f64, t13698: f64, t4415: f64, t12863: f64, t3273: f64, t5387: f64, t1213: f64, t12835: f64, t12846: f64, t12861: f64, t12881: f64, t12889: f64, t13725: f64, t13727: f64, t13731: f64, t13736: f64, t13741: f64, t3271: f64, t4413: f64, t9995: f64) -> (f64, f64, f64) {
    let t13745 = t4415 * t13698 * t1233;
    let t13749 = t3273 * t12863 * t5387;
    let t13752 = 7.0_f64 / 4608.0_f64 * t13725 - 7.0_f64 / 2304.0_f64 * t13727 - t12835 - 119.0_f64 / 6912.0_f64 * t12846 - t1213 * t13731 / 48.0_f64 - 119.0_f64 / 3456.0_f64 * t9995 - t4413 * t13736 / 192.0_f64 - 35.0_f64 / 108.0_f64 * t12861 - t12881 - t12889 + t3271 * t13741 / 768.0_f64 - t3271 * t13745 / 3072.0_f64 + t3271 * t13749 / 384.0_f64;
    (t13745, t13749, t13752)
}
