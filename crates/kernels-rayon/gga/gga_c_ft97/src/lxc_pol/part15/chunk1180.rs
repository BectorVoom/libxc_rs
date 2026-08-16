//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1180/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1180(t1701: f64, t17975: f64, t5295: f64, t5284: f64, t1196: f64, t2035: f64, t22090: f64, t5260: f64, t21249: f64, t21253: f64, t290: f64, t1111: f64, t1472: f64, t14742: f64, t19107: f64, t19132: f64, t22136: f64, t22154: f64, t284: f64, t28676: f64, t291: f64, t4094: f64, t4099: f64, t5003: f64, t90075: f64, t90081: f64, t90085: f64, t90153: f64) -> (f64, f64, f64, f64, f64) {
    let t90172 = t1701 * t17975 * t5295;
    let t90186 = t1701 * t17975 * t5284;
    let t90192 = t2035 * t22090 * t1196;
    let t90195 = t5260 * t5260;
    let t90200 = t21249 * t21253;
    let t90201 = t290 * t1196 * t90200;
    let t90204 = 0.22955470875934553164e2_f64 * t1472 * t90172 + 0.4832730710723063824e1_f64 * t22136 * t1111 - 0.2416365355361531912e1_f64 * t1472 * t90153 + 0.17516464591774387197e2_f64 * t19107 * t90075 - 0.22445349300913785316e3_f64 * t4094 * t90081 + 0.11222674650456892658e3_f64 * t4099 * t90085 + 0.45910941751869106328e2_f64 * t14742 * t90186 + 0.45910941751869106328e2_f64 * t22154 * t5003 - 0.22187521816247557116e3_f64 * t19132 * t90192 + 6.0_f64 * t90195 * t284 * t291 - 0.2607118765118496554e1_f64 * t28676 * t90201;
    (t90172, t90186, t90192, t90201, t90204)
}
