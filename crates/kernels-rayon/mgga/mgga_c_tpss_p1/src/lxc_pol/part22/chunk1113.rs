//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1113/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1113(t926: f64, t9637: f64, t11878: f64, t140: f64, t3032: f64, t4047: f64, t1098: f64, t1100: f64, t4052: f64, t11888: f64, t4219: f64, t11894: f64) -> (f64, f64, f64, f64, f64) {
    let t12278 = t926 * t9637;
    let t12279 = t12278 * t11878;
    let t12287 = t140 * t3032;
    let t12288 = t12287 * t4047;
    let t12290 = t1098 * t12288 / 324.0_f64;
    let t12291 = t140 * t1100;
    let t12292 = t12291 * t4052;
    let t12294 = t1098 * t12292 / 216.0_f64;
    let t12295 = t4219 * t11888;
    let t12298 = t4219 * t11894;
    (t12279, t12290, t12294, t12295, t12298)
}
