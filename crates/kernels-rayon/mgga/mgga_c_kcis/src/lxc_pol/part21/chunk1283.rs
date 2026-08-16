//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1283/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1283(t1003: f64, t13376: f64, t26686: f64, t14570: f64, t283: f64, t990: f64, t1008: f64, t2811: f64, t4972: f64, t27778: f64, t3045: f64, t1020: f64, t26675: f64, t27836: f64) -> (f64, f64, f64, f64, f64) {
    let t95636 = t26686 * t13376 * t1003;
    let t95640 = t14570 * t283 * t990;
    let t95645 = t26686 * t2811 * t4972 * t1008;
    let t95649 = t26686 * t27778 * t3045;
    let t95653 = t1020 * t27836 * t26675;
    (t95636, t95640, t95645, t95649, t95653)
}
