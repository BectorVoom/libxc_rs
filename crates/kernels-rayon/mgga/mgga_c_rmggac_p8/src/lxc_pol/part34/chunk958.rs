//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 958/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk958(t74333: f64, t74337: f64, t74339: f64, t74345: f64, t74354: f64, t74356: f64, t74368: f64, t74371: f64, t74374: f64, t3351: f64, t3352: f64, t875: f64, t9577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t76972 = 0.15961724959986689775e-4_f64 * t74333;
    let t76973 = 0.2553875993597870364e-4_f64 * t74337;
    let t76974 = 0.1702583995731913576e-4_f64 * t74339;
    let t76975 = 0.1702583995731913576e-4_f64 * t74345;
    let t76976 = 0.1702583995731913576e-4_f64 * t74354;
    let t76977 = 0.85129199786595678799e-5_f64 * t74356;
    let t76978 = 0.85129199786595678799e-5_f64 * t74368;
    let t76979 = 0.15961724959986689775e-4_f64 * t74371;
    let t76980 = 0.1276937996798935182e-4_f64 * t74374;
    let t76985 = t3351 * t3352 * t875 * t9577;
    (t76972, t76973, t76974, t76975, t76976, t76977, t76978, t76979, t76980, t76985)
}
