//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1353/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1353(t10667: f64, t10703: f64, t2289: f64, t2292: f64, t24799: f64, t24996: f64, t260: f64, t28949: f64, t29068: f64, t29071: f64, t29072: f64, t29079: f64, t29081: f64, t29128: f64, t29177: f64, t29214: f64, t29316: f64, t29337: f64, t29380: f64, t29384: f64, t29387: f64, t29423: f64, t29478: f64, t3422: f64, t3443: f64, t3444: f64, t849: f64, t856: f64) -> f64 {
    let t29493 = 0.14035736694323150897e2_f64 * t856 * t10667 * t2292 + t29068 + t29071 - 0.34631718211362927518e2_f64 * t856 * t29072 * t3444 - t29079 - t29081 + t260 * (t29128 + t29177 + t29214 + t29316 + t29337 + t29380 + t29423 + t29478) + 0.23392894490538584828e1_f64 * t856 * t2289 * t10703 * t849 + 0.4155806185363551302e3_f64 * t24799 * t3422 * t28949 - 0.34631718211362927518e2_f64 * t856 * t3443 * t24996 + t29384 - t29387;
    t29493
}
