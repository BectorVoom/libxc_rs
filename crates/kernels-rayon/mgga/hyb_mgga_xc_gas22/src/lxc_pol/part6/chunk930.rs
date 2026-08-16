//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 930/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk930(t1270: f64, t1282: f64, t172: f64, t184: f64, t2104: f64, t2112: f64, t2116: f64, t2133: f64, t2144: f64, t3227: f64, t3231: f64, t3232: f64, t3264: f64, t6363: f64, t740: f64, t742: f64, t756: f64, t8354: f64, t8367: f64, t8370: f64, t8373: f64, t8431: f64) -> f64 {
    let t8434 = 7.0_f64 / 2.0_f64 * t2133 * t3232 - t8367 * t3232 / 2.0_f64 - t8370 * t3232 / 4.0_f64 - t3231 * t8373 - 6.0_f64 * t6363 * t1270 * t2112 + 4.0_f64 * t2116 * t3227 * t740 + 2.0_f64 * t2116 * t1270 * t2104 - t742 * t8354 + 2.0_f64 * t8354 * t184 + 4.0_f64 * t3227 * t756 + 2.0_f64 * t1270 * t2144 + 2.0_f64 * t2104 * t1282 + 4.0_f64 * t740 * t3264 + 2.0_f64 * t172 * t8431;
    t8434
}
