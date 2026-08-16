//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 920/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk920(t1907: f64, t1971: f64, t352: f64, t515: f64, t7230: f64, t7717: f64, t9783: f64, t39277: f64, t9123: f64, t9206: f64, t10014: f64, t36662: f64) -> (f64, f64, f64, f64, f64) {
    let t45305 = t7230 * t1971 * t515 * t1907 * t352;
    let t45307 = t7717 * t9783;
    let t45309 = t39277 * t9123;
    let t45316 = t39277 * t9206;
    let t45318 = t36662 * t10014;
    (t45305, t45307, t45309, t45316, t45318)
}
