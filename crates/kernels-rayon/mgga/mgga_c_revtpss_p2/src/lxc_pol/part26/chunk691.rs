//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 691/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk691(t508: f64, t7373: f64, t2089: f64, t670: f64, t2061: f64, t212: f64, t780: f64, t689: f64, t2062: f64, t786: f64, t789: f64, t7023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7374 = t508 * t7373;
    let t7378 = t2089 * t670;
    let t7384 = t212 * t2061;
    let t7385 = t7384 * t780;
    let t7387 = 0.54878743191129263322e-2_f64 * t689 * t7385;
    let t7388 = t786 * t2062;
    let t7390 = 0.9757440539382783019e-2_f64 * t7388 * t789;
    let t7391 = 7.0_f64 / 144.0_f64 * t7023;
    (t7374, t7378, t7384, t7385, t7387, t7388, t7390, t7391)
}
