//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1309/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1309(t10350: f64, t10364: f64, t10373: f64, t10394: f64, t10408: f64, t1270: f64, t20530: f64, t20545: f64, t2104: f64, t2112: f64, t2116: f64, t2133: f64, t24354: f64, t28549: f64, t3231: f64, t4046: f64, t4051: f64, t4068: f64, t6355: f64, t6363: f64, t740: f64, t8354: f64, t8367: f64, t8370: f64, t8395: f64, t8396: f64) -> f64 {
    let t28623 = 7.0_f64 / 2.0_f64 * t4068 * t6355 + 15.0_f64 / 4.0_f64 * t10408 * t8396 - t8395 * t24354 - t10364 * t6355 / 4.0_f64 - t20545 * t4051 * t8396 / 8.0_f64 - 6.0_f64 * t6363 * t4051 * t2104 + 4.0_f64 * t2116 * t1270 * t8354 - t8367 * t10373 / 2.0_f64 - t3231 * t28549 - t8370 * t10373 / 4.0_f64 + 4.0_f64 * t2116 * t10350 * t740 + 2.0_f64 * t2116 * t4046 * t2104 - 24.0_f64 * t10394 * t8396 + 24.0_f64 * t20530 * t4051 * t2112 + 7.0_f64 / 2.0_f64 * t2133 * t10373 - 6.0_f64 * t6363 * t4046 * t2112;
    t28623
}
