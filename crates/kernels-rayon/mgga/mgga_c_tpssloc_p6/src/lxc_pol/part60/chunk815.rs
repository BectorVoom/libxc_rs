//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 815/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk815(t2039: f64, t6287: f64, t2075: f64, t5493: f64, t1774: f64, t7801: f64, t19596: f64, t2095: f64, t1268: f64, t1458: f64, t19451: f64, t27188: f64, t28002: f64, t28007: f64, t28943: f64, t28951: f64, t28959: f64, t4028: f64, t7042: f64, t7676: f64) -> (f64, f64, f64, f64, f64) {
    let t29211 = t6287 * t2039;
    let t29214 = t2075 * t5493;
    let t29219 = t1774 * t7801;
    let t29222 = t2095 * t19596;
    let t29241 = 2.0_f64 * t1268 * t28951 + 4.0_f64 * t1458 * t27188 + 2.0_f64 * t19451 * t2039 + 4.0_f64 * t2039 * t28002 + 2.0_f64 * t2039 * t28007 + 4.0_f64 * t4028 * t7801 + 2.0_f64 * t5493 * t7042 + 4.0_f64 * t7676 * t7801 + t28943 + 2.0_f64 * t28959;
    (t29211, t29214, t29219, t29222, t29241)
}
