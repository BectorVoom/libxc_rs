//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 936/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk936(t352: f64, t9888: f64, t262: f64, t36634: f64, t10093: f64, t495: f64, t515: f64, t7230: f64, t7231: f64, t10082: f64, t3351: f64, t7248: f64) -> (f64, f64, f64, f64, f64) {
    let t45577 = t9888 * t352;
    let t45578 = t262 * t45577;
    let t45579 = t36634 * t45578;
    let t45584 = t7230 * t7231 * t515 * t10093 * t495;
    let t45589 = t3351 * t7248 * t515 * t10082 * t352;
    (t45577, t45578, t45579, t45584, t45589)
}
