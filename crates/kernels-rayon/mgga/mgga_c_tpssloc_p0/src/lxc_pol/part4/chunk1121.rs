//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1121/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1121(t3450: f64, t5398: f64, t3449: f64, t18237: f64, t4908: f64, t3448: f64, t6138: f64, t3451: f64, t6144: f64, t18225: f64, t11583: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18409 = t3450 * t5398;
    let t18410 = t3449 * t18409;
    let t18413 = t4908 * t18237;
    let t18416 = t3448 * t6138;
    let t18417 = t18416 * t3451;
    let t18420 = t3448 * t6144;
    let t18421 = t18420 * t3451;
    let t18424 = t4908 * t18225;
    let t18427 = t11583 * t5392;
    (t18410, t18413, t18417, t18421, t18424, t18427)
}
