//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 892/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk892(t1213: f64, t18375: f64, t248: f64, t3521: f64, t5975: f64, t1227: f64, t3450: f64, t5398: f64, t3448: f64, t6138: f64, t6144: f64, t11583: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18376 = t1213 * t18375;
    let t18392 = t248 * t3521 * t5975;
    let t18393 = t1227 * t18392;
    let t18409 = t3450 * t5398;
    let t18416 = t3448 * t6138;
    let t18420 = t3448 * t6144;
    let t18427 = t11583 * t5392;
    (t18376, t18393, t18409, t18416, t18420, t18427)
}
