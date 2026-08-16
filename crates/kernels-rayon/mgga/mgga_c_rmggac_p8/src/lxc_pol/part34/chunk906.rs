//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 906/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk906(t25854: f64, t75848: f64, t27048: f64, t75851: f64, t14305: f64, t75303: f64, t1326: f64, t14309: f64, t2048: f64, t570: f64, t2079: f64, t2367: f64, t262: f64, t265: f64) -> (f64, f64, f64, f64, f64) {
    let t76222 = 0.17961362552795712846e0_f64 * t25854 * t75848;
    let t76224 = 0.17961362552795712846e0_f64 * t27048 * t75851;
    let t76228 = t14305 * t75303;
    let t76232 = t14309 * t1326 * t2048 * t570;
    let t76236 = t2079 * t262 * t265 * t2367;
    (t76222, t76224, t76228, t76232, t76236)
}
