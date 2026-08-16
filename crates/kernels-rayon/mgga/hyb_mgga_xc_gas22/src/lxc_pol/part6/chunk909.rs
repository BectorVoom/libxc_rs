//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 909/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk909(t3023: f64, t35: f64, t572: f64, t6007: f64, t6010: f64, t6013: f64, t6015: f64, t6017: f64, t6019: f64, t7933: f64, t7936: f64, t7938: f64, t7943: f64, t7948: f64, t7953: f64, t7958: f64, t7962: f64, t7966: f64, t7971: f64, t7975: f64, t7979: f64) -> f64 {
    let t7983 = -t6010 - 4.0_f64 / 243.0_f64 * t6013 + t6015 / 243.0_f64 - t6017 / 81.0_f64 + t6019 / 162.0_f64 - 2.0_f64 / 243.0_f64 * t7933 + t7936 - t7938 + 11.0_f64 / 81.0_f64 * t7943 - 5.0_f64 / 243.0_f64 * t572 * t7948 + 2.0_f64 / 27.0_f64 * t572 * t7953 - 4.0_f64 / 81.0_f64 * t3023 * t7958 - t572 * t7962 / 81.0_f64 - t572 * t7966 / 9.0_f64 + 4.0_f64 / 27.0_f64 * t3023 * t7971 + t572 * t7975 / 27.0_f64 - t35 * t6007 * t7979 / 27.0_f64;
    t7983
}
