//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 540/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk540(t2046: f64, t2051: f64, t7297: f64, t270: f64, t303: f64, t2039: f64, t638: f64, t357: f64, t36: f64, t4789: f64, t71: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7299 = t2046 * t7297 * t2051;
    let t7301 = t303 * t270;
    let t7303 = t638 * t2039 * t7301;
    let t7305 = t357 * t270;
    let t7307 = t638 * t2039 * t7305;
    let t7310 = t36 * t4789 * t71;
    (t7299, t7301, t7303, t7305, t7307, t7310)
}
