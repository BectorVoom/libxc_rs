//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 919/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk919(t39570: f64, t8636: f64, t39705: f64, t8902: f64, t17859: f64, t9213: f64, t9218: f64, t1907: f64, t1971: f64, t333: f64, t511: f64, t7230: f64) -> (f64, f64, f64, f64, f64) {
    let t45289 = t39570 * t8636;
    let t45291 = t39705 * t8902;
    let t45293 = t17859 * t9213;
    let t45295 = t17859 * t9218;
    let t45300 = t7230 * t1971 * t511 * t1907 * t333;
    (t45289, t45291, t45293, t45295, t45300)
}
