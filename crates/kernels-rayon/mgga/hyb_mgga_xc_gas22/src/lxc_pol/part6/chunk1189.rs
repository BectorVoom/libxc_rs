//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1189/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1189(t1110: f64, t2637: f64, t2806: f64, t222: f64, t2702: f64, t7436: f64, t1030: f64, t1048: f64, t3021: f64, t1884: f64, t2711: f64, t2714: f64) -> (f64, f64, f64, f64) {
    let t22030 = 0.21053605041484726346e2_f64 * t1110 * t2637 * t2806;
    let t22033 = 0.71233333333333333332e-1_f64 * t222 * t2702 * t7436;
    let t22038 = 0.22161481481481481481e0_f64 * t222 * t3021 * t1030 * t1048;
    let t22042 = 0.28493333333333333333e0_f64 * t222 * t1884 * t2711 * t2714;
    (t22030, t22033, t22038, t22042)
}
