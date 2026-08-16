//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 876/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk876(t1550: f64, t2060: f64, t44713: f64, t558: f64, t8708: f64, t262: f64, t7198: f64, t570: f64, t7204: f64, t34884: f64, t9971: f64, t16503: f64, t35039: f64, t571: f64, t8420: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44727 = t1550 * t2060 * t44713;
    let t44732 = t8708 * t558;
    let t44733 = t262 * t44732;
    let t44734 = t7198 * t44733;
    let t44736 = t8708 * t570;
    let t44737 = t262 * t44736;
    let t44738 = t7204 * t44737;
    let t44740 = t34884 * t9971;
    let t44744 = t16503 * t35039 * t571 * t8420;
    (t44727, t44732, t44733, t44734, t44736, t44737, t44738, t44740, t44744)
}
