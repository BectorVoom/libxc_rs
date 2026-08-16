//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1162/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1162(t248: f64, t3521: f64, t5975: f64, t1227: f64, t1409: f64, t15701: f64, t15700: f64, t3578: f64, t1735: f64, t4729: f64, t18232: f64, t4900: f64) -> (f64, f64, f64, f64) {
    let t18392 = t248 * t3521 * t5975;
    let t18393 = t1227 * t18392;
    let t18395 = t15701 * t1409;
    let t18396 = t15700 * t18395;
    let t18397 = t3578 * t18396;
    let t18400 = t1735 * t4729;
    let t18401 = t3578 * t18400;
    let t18404 = t4900 * t18232;
    (t18393, t18397, t18401, t18404)
}
