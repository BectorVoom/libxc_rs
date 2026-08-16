//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 620/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk620(t300: f64, t3527: f64, t3489: f64, t1175: f64, t1198: f64, t1188: f64, t3495: f64, t3497: f64, t1196: f64, t1179: f64, t3515: f64, t3520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3528 = t300 * t3527;
    let t3530 = 0.19751673498613801407e-1_f64 * t300 * t3489;
    let t3531 = t300 * t1175;
    let t3533 = 0.11696447245269292414e1_f64 * t3531 * t1198;
    let t3535 = t3495 * t3497 * t1188;
    let t3537 = 0.11696447245269292414e1_f64 * t1196 * t3535;
    let t3539 = t1179 * t3515 * t1188;
    let t3541 = 0.5848223622634646207e0_f64 * t1196 * t3539;
    let t3542 = t3520 * t3497;
    (t3528, t3530, t3531, t3533, t3535, t3537, t3539, t3541, t3542)
}
