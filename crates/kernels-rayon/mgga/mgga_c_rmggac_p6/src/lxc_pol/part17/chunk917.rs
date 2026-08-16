//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 917/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk917(t39277: f64, t9206: f64, t10014: f64, t36662: f64, t1907: f64, t236: f64, t321: f64, t3352: f64, t7230: f64, t17859: f64, t9106: f64, t9111: f64) -> (f64, f64, f64, f64, f64) {
    let t45316 = t39277 * t9206;
    let t45318 = t36662 * t10014;
    let t45323 = t7230 * t3352 * t236 * t1907 * t321;
    let t45325 = t17859 * t9106;
    let t45327 = t17859 * t9111;
    (t45316, t45318, t45323, t45325, t45327)
}
