//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1133/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1133(t10850: f64, t11217: f64, t493: f64, t7244: f64, t7251: f64, t7257: f64, t7258: f64, t7263: f64, t7267: f64, t7271: f64, t9319: f64, t9323: f64, t9325: f64, t9329: f64, t9330: f64, t9334: f64, t9336: f64) -> f64 {
    let t11224 = -t10850 - t7244 - t7251 + t7257 + 0.11696447245269292414e1_f64 * t7258 + 0.19751673498613801407e-1_f64 * t11217 * t493 + 0.48830526149350786811e-3_f64 * t9319 - t9323 + 0.21687162600603479684e-1_f64 * t9325 + t7263 + t7267 + t7271 + t9329 - 24.0_f64 * t9330 + 40.0_f64 * t9334 - t9336;
    t11224
}
