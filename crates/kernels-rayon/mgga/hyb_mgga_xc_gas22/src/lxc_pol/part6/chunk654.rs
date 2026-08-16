//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 654/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk654(t1238: f64, t2022: f64, t3177: f64, t675: f64, t3: f64, t699: f64, t702: f64, t2047: f64, t2048: f64, t3023: f64, t3169: f64, t3174: f64, t572: f64) -> (f64, f64, f64, f64) {
    let t3178 = t2022 * t1238;
    let t3180 = t3177 * t3178 * t675;
    let t3184 = t699 * t702 * t3;
    let t3187 = t2047 + t2048 / 162.0_f64 + t3169 / 162.0_f64 - t572 * t3174 / 81.0_f64 + t572 * t3180 / 27.0_f64 + t3023 * t3184 / 27.0_f64;
    (t3178, t3180, t3184, t3187)
}
