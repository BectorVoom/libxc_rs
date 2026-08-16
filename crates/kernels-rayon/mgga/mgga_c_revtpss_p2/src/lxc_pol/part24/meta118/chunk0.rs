//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 653/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk653(t1568: f64, t212: f64, t780: f64, t689: f64, t1569: f64, t786: f64, t789: f64, t1469: f64, t80: f64, t83: f64, t1544: f64, t221: f64, t2675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4321 = t212 * t1568;
    let t4322 = t4321 * t780;
    let t4323 = t689 * t4322;
    let t4325 = t786 * t1569;
    let t4326 = t4325 * t789;
    let t4328 = t80 * t1469;
    let t4335 = t83 * t1469;
    let t4349 = t2675 * t221 * t1544;
    (t4321, t4322, t4323, t4325, t4326, t4328, t4335, t4349)
}
