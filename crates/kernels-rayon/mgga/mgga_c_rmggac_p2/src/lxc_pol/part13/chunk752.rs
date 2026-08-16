//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 752/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk752(t31: f64, t356: f64, t290: f64, t2012: f64, t7349: f64, t2019: f64, t35214: f64, t640: f64, t7764: f64, t7553: f64, t7555: f64, t1302: f64, t131: f64, t1310: f64, t20: f64, t2018: f64, t2020: f64, t252: f64) -> (f64, f64, f64, f64, f64) {
    let t35219 = t356 * t31;
    let t35220 = t290 * t35219;
    let t35222 = t7349 * t2012 * t35220;
    let t35226 = t2019 * t7764 * t640 * t35214;
    let t35228 = t640 * t35219;
    let t35230 = t7553 * t7555 * t35228;
    let t35238 = t1310 * t252 * t20 * t2018 * t2020 * t640 * t131 * t1302;
    (t35220, t35222, t35226, t35230, t35238)
}
