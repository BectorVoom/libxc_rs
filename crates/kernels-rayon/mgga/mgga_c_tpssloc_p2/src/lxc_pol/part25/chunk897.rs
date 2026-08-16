//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 897/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk897(t11516: f64, t9288: f64, t3440: f64, t3441: f64, t1177: f64, t1178: f64, t9258: f64, t1176: f64, t698: f64, t1179: f64, t1174: f64, t3431: f64, t3460: f64) -> (f64, f64, f64, f64, f64) {
    let t11517 = t11516 * t9288;
    let t11518 = t3440 * t11517;
    let t11521 = t3441 * t9288;
    let t11522 = t1177 * t11521;
    let t11525 = t1178 * t9258;
    let t11526 = t1177 * t11525;
    let t11529 = t698 * t1176;
    let t11530 = t11529 * t1179;
    let t11531 = t1174 * t11530;
    let t11533 = t3431 * t3460;
    (t11518, t11522, t11526, t11531, t11533)
}
