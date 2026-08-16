//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 946/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk946(t7349: f64, t7760: f64, t9719: f64, t1587: f64, t2347: f64, t262: f64, t8640: f64, t34724: f64, t9709: f64, t558: f64, t8704: f64, t7198: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45716 = t7349 * t9719 * t7760;
    let t45720 = t2347 * t1587;
    let t45721 = t262 * t45720;
    let t45722 = t8640 * t45721;
    let t45724 = t34724 * t9709;
    let t45726 = t8704 * t558;
    let t45727 = t262 * t45726;
    let t45728 = t7198 * t45727;
    (t45716, t45720, t45721, t45722, t45724, t45726, t45727, t45728)
}
