//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 931/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk931(t143: f64, t1270: f64, t1285: f64, t172: f64, t187: f64, t2104: f64, t2147: f64, t3227: f64, t3267: f64, t740: f64, t759: f64, t8266: f64, t8352: f64, t8354: f64, t8434: f64) -> f64 {
    let t144 = 0.135e1_f64 <= t143;
    let t8438 = piecewise3(t144, t8266 + t8352, -8.0_f64 / 3.0_f64 * t8354 * t187 - 16.0_f64 / 3.0_f64 * t3227 * t759 - 8.0_f64 / 3.0_f64 * t1270 * t2147 - 8.0_f64 / 3.0_f64 * t2104 * t1285 - 16.0_f64 / 3.0_f64 * t740 * t3267 - 8.0_f64 / 3.0_f64 * t172 * t8434);
    t8438
}
