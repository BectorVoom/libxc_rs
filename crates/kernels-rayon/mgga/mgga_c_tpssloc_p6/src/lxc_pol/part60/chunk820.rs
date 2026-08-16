//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 820/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk820(t29286: f64, t553: f64, t24127: f64, t6388: f64, t1336: f64, t1814: f64, t2089: f64, t24099: f64, t26381: f64, t26393: f64, t26406: f64, t28132: f64, t28136: f64, t28140: f64, t28144: f64, t28150: f64, t544: f64, t6378: f64, t7934: f64) -> f64 {
    let t29327 = t553 * t29286;
    let t29339 = t24127 * t6388;
    let t29342 = 0.15352717957250113407e0_f64 * t26381 - t24099 + t544 * t29327 + 0.3289868133696452873e-1_f64 * t26393 + 2.0_f64 * t1814 * t7934 + 0.6579736267392905746e-1_f64 * t28132 + t6378 * t2089 + 0.3289868133696452873e-1_f64 * t28136 + 0.76763589786250567036e-1_f64 * t26406 - 0.3289868133696452873e-1_f64 * t28140 + 0.9869604401089358619e-1_f64 * t28144 - 0.6579736267392905746e-1_f64 * t28150 + 2.0_f64 * t1336 * t29339;
    t29342
}
