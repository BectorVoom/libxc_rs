//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1208/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1208(t39900: f64, t39903: f64, t39908: f64, t39912: f64, t39920: f64, t41609: f64, t43407: f64, t43410: f64, t43413: f64, t43415: f64, t43418: f64, t43421: f64) -> f64 {
    let t43423 = -t39900 - 0.13972381860938637374e0_f64 * t39903 + t41609 - 0.65854491829355115985e-1_f64 * t39908 - t39912 - 0.86682217400542685632e-1_f64 * t43407 + 0.43663693315433241792e-2_f64 * t43410 + 0.13099107994629972538e-1_f64 * t43413 - 0.87327386630866483584e-2_f64 * t43415 + 0.14282990759302185292e-1_f64 * t39920 + 0.23115257973478049502e0_f64 * t43418 + 0.11557628986739024751e0_f64 * t43421;
    t43423
}
