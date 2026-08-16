//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1075/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1075(t10350: f64, t10364: f64, t10373: f64, t10424: f64, t1270: f64, t1282: f64, t172: f64, t184: f64, t2116: f64, t3227: f64, t3231: f64, t3246: f64, t3264: f64, t4046: f64, t4051: f64, t4068: f64, t4079: f64, t6363: f64, t740: f64, t742: f64, t756: f64, t8373: f64, t8395: f64) -> f64 {
    let t10427 = 7.0_f64 / 2.0_f64 * t4068 * t3246 - t8395 * t8373 - t10364 * t3246 / 4.0_f64 - 6.0_f64 * t6363 * t4051 * t740 + 4.0_f64 * t2116 * t1270 * t3227 - t3231 * t10373 / 2.0_f64 + 2.0_f64 * t2116 * t4046 * t740 - t742 * t10350 + 2.0_f64 * t10350 * t184 + 2.0_f64 * t4046 * t756 + 4.0_f64 * t3227 * t1282 + 4.0_f64 * t1270 * t3264 + 2.0_f64 * t740 * t4079 + 2.0_f64 * t172 * t10424;
    t10427
}
