//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 921/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk921(t1907: f64, t236: f64, t321: f64, t3352: f64, t7230: f64, t17859: f64, t9106: f64, t9111: f64, t2283: f64, t38472: f64, t2286: f64, t38638: f64) -> (f64, f64, f64, f64, f64) {
    let t45323 = t7230 * t3352 * t236 * t1907 * t321;
    let t45325 = t17859 * t9106;
    let t45327 = t17859 * t9111;
    let t45329 = t38472 * t2283;
    let t45331 = t38638 * t2286;
    (t45323, t45325, t45327, t45329, t45331)
}
