//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 949/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk949(t11873: f64, t11910: f64, t11942: f64, t1072: f64, t4155: f64, t1535: f64, t2998: f64, t1523: f64, t2929: f64, t2973: f64, t11844: f64, t11875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12035 = 0.22954444444444444444e0_f64 * t11873;
    let t12046 = 0.27785333333333333334e0_f64 * t11910;
    let t12060 = 0.34431666666666666666e0_f64 * t11942;
    let t12070 = t4155 * t1072;
    let t12075 = t1535 * t2998;
    let t12083 = t1523 * t2929;
    let t12086 = t1535 * t2973;
    let t12093 = 0.11038e0_f64 * t11844;
    let t12104 = 0.13418888888888888889e0_f64 * t11873;
    let t12115 = 0.22076e0_f64 * t11910;
    let t12129 = 0.20128333333333333334e0_f64 * t11942;
    let t12145 = 0.2283111111111111111e-1_f64 * t11875;
    (t12035, t12046, t12060, t12070, t12075, t12083, t12086, t12093, t12104, t12115, t12129, t12145)
}
