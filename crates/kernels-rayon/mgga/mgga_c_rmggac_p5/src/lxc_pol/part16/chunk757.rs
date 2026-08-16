//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 757/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk757(t1550: f64, t35583: f64, t333: f64, t7817: f64, t903: f64, t338: f64, t830: f64, t352: f64, t739: f64, t131: f64, t1338: f64, t2019: f64, t640: f64, t7764: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35584 = t1550 * t35583;
    let t35586 = t7817 * t333;
    let t35587 = t903 * t35586;
    let t35589 = t338 * t830;
    let t35590 = t35589 * t352;
    let t35591 = t739 * t35590;
    let t35604 = t131 * t1338;
    let t35607 = t2019 * t7764 * t640 * t35604;
    (t35584, t35586, t35587, t35589, t35590, t35591, t35604, t35607)
}
