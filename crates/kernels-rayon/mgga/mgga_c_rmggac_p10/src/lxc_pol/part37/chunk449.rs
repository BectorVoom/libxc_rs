//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 449/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk449(t511: f64, t9157: f64, t209: f64, t476: f64, t570: f64, t515: f64, t618: f64, t236: f64, t498: f64, t551: f64, t107: f64, t500: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9158 = t511 * t9157;
    let t9163 = t570 * t476 * t209;
    let t9164 = t515 * t9163;
    let t9169 = t618 * t476 * t209;
    let t9170 = t236 * t9169;
    let t9182 = t551 * t498;
    let t9183 = t236 * t9182;
    let t9187 = t500 * t107;
    (t9158, t9163, t9164, t9169, t9170, t9183, t9187)
}
