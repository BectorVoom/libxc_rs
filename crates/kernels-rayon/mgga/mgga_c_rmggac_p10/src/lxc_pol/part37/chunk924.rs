//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 924/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk924(t76717: f64, t15502: f64, t3351: f64, t498: f64, t515: f64, t9210: f64, t321: f64, t7248: f64, t2144: f64, t333: f64, t7231: f64, t352: f64, t875: f64) -> (f64, f64, f64, f64, f64) {
    let t76718 = 0.15961724959986689774e-4_f64 * t76717;
    let t76722 = t3351 * t9210 * t515 * t15502 * t498;
    let t76723 = 0.85129199786595678796e-5_f64 * t76722;
    let t76727 = t3351 * t7248 * t515 * t15502 * t321;
    let t76728 = 0.12769379967989351819e-4_f64 * t76727;
    let t76732 = t3351 * t7231 * t2144 * t15502 * t333;
    let t76733 = 0.12769379967989351819e-4_f64 * t76732;
    let t76737 = t3351 * t7231 * t875 * t15502 * t352;
    (t76718, t76723, t76728, t76733, t76737)
}
