//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1238/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1238(t1014: f64, t26843: f64, t3245: f64, t7727: f64, t7735: f64, t1094: f64, t3169: f64, t26773: f64, t26778: f64, t26797: f64, t26848: f64, t27084: f64, t7784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92991 = t1014 * t26843;
    let t92993 = t3245 * t7727;
    let t92997 = t3245 * t7735;
    let t92999 = t3169 * t1094;
    let t93006 = t1014 * t26773;
    let t93008 = t1014 * t26778;
    let t93010 = t1014 * t26797;
    let t93012 = t1014 * t26848;
    let t93014 = t27084 * t7784;
    (t92991, t92993, t92997, t92999, t93006, t93008, t93010, t93012, t93014)
}
