//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 519/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk519(t45: f64, t1522: f64, t2398: f64, t1568: f64, t212: f64, t780: f64, t689: f64, t1569: f64, t786: f64, t789: f64, t1469: f64, t80: f64, t4186: f64, t606: f64, t766: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t4316 = 4.0_f64 * t2398 * t1522;
    let t4321 = t212 * t1568;
    let t4322 = t4321 * t780;
    let t4323 = t689 * t4322;
    let t4325 = t786 * t1569;
    let t4326 = t4325 * t789;
    let t4328 = t80 * t1469;
    let t4334 = piecewise3(t151, 0.0_f64, -2.0_f64 / 9.0_f64 * t4328 * t606 + 2.0_f64 / 3.0_f64 * t766 * t4186);
    (t4316, t4323, t4326, t4334)
}
