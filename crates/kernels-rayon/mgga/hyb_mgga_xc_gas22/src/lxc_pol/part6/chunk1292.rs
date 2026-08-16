//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1292/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1292(t10460: f64, t677: f64, t10433: f64, t136: f64, t550: f64, t2011: f64, t3990: f64, t1815: f64, t3985: f64, t20216: f64, t2024: f64, t3926: f64) -> (f64, f64, f64, f64, f64) {
    let t28046 = t677 * t10460;
    let t28049 = t136 * t550 * t10433;
    let t28057 = t3990 * t2011;
    let t28060 = t136 * t1815 * t3985;
    let t28066 = t2024 * t20216 * t3926;
    (t28046, t28049, t28057, t28060, t28066)
}
