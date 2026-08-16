//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 104/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk104(t238: f64, t242: f64, t243: f64, t228: f64, t231: f64, t234: f64) -> (f64, f64, f64, f64) {
    let t245 = t238 * t242 * t243;
    let t247 = 0.379785e1_f64 * t231 + 0.8969e0_f64 * t228 + 0.204775e0_f64 * t234 + 0.123235e0_f64 * t245;
    let t250 = 1.0_f64 + 0.16081979498692535067e2_f64 / t247;
    let t251 = f64::ln(t250);
    (t245, t247, t250, t251)
}
