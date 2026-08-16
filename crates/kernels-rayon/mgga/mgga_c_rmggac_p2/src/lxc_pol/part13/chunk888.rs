//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 888/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk888(t39689: f64, t7198: f64, t333: f64, t8708: f64, t262: f64, t352: f64, t7204: f64, t1614: f64, t2064: f64, t903: f64, t1679: f64, t7203: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39690 = t7198 * t39689;
    let t39692 = t8708 * t333;
    let t39693 = t262 * t39692;
    let t39694 = t7198 * t39693;
    let t39696 = t8708 * t352;
    let t39697 = t262 * t39696;
    let t39698 = t7204 * t39697;
    let t39700 = t2064 * t1614;
    let t39701 = t903 * t39700;
    let t39705 = t1679 * t7203;
    (t39690, t39692, t39693, t39694, t39696, t39697, t39698, t39700, t39701, t39705)
}
