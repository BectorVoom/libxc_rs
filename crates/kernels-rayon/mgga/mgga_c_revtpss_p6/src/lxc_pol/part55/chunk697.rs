//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 697/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk697(t2089: f64, t670: f64, t2061: f64, t212: f64, t780: f64, t689: f64, t2062: f64, t786: f64, t789: f64, t7023: f64, t7031: f64, t7034: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7378 = t2089 * t670;
    let t7384 = t212 * t2061;
    let t7385 = t7384 * t780;
    let t7387 = 0.54878743191129263322e-2_f64 * t689 * t7385;
    let t7388 = t786 * t2062;
    let t7390 = 0.9757440539382783019e-2_f64 * t7388 * t789;
    let t7391 = 7.0_f64 / 144.0_f64 * t7023;
    let t7393 = 0.28582678745379824648e-4_f64 * t7031;
    let t7394 = 0.50820002809285328225e-4_f64 * t7034;
    (t7378, t7384, t7385, t7387, t7388, t7390, t7391, t7393, t7394)
}
