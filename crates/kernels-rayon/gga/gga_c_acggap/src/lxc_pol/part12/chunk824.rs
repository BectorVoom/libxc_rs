//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 824/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk824(t8680: f64, t8682: f64, t8684: f64, t8690: f64, t8694: f64, t7384: f64, t7388: f64, t7391: f64, t7397: f64, t7406: f64, t8686: f64, t8692: f64, t8696: f64, t8698: f64, t8700: f64) -> f64 {
    let t9248 = 11.0_f64 / 192.0_f64 * t8680;
    let t9249 = 11.0_f64 / 576.0_f64 * t8682;
    let t9250 = 7.0_f64 / 72.0_f64 * t8684;
    let t9252 = 0.21437009059034868486e-3_f64 * t8690;
    let t9254 = 0.17149607247227894789e-2_f64 * t8694;
    let t9258 = -t7384 - t7388 - t7391 + t7397 + t7406 + t9248 + t9249 + t9250 + 0.51448821741683684367e-2_f64 * t8686 - t9252 - 0.34299214494455789578e-2_f64 * t8692 + t9254 + 0.68598428988911579156e-2_f64 * t8696 + 0.17149607247227894789e-1_f64 * t8698 - 0.68598428988911579156e-2_f64 * t8700;
    t9258
}
