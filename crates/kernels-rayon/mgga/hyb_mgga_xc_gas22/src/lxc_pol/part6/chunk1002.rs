//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1002/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1002(t2631: f64, t9324: f64, t1100: f64, t3636: f64, t462: f64, t1524: f64, t2754: f64, t2757: f64, t2751: f64, t7272: f64, t7244: f64, t7246: f64, t7251: f64, t7257: f64, t7258: f64, t7263: f64, t7267: f64, t7271: f64, t9319: f64, t9323: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9325 = t9324 * t2631;
    let t9327 = t3636 * t1100;
    let t9329 = 2.0_f64 * t462 * t9327;
    let t9330 = t2754 * t1524;
    let t9332 = t2757 * t1524;
    let t9334 = t2751 * t1524;
    let t9336 = 12.0_f64 * t7272;
    let t9337 = -t7244 - 0.5848223622634646207e0_f64 * t7246 - t7251 + t7257 + 0.23392894490538584828e1_f64 * t7258 + 0.24415263074675393405e-3_f64 * t9319 - t9323 + 0.10843581300301739842e-1_f64 * t9325 + t7263 + t7267 + t7271 + t9329 + 12.0_f64 * t9330 - 32.0_f64 * t9332 + 20.0_f64 * t9334 - t9336;
    (t9325, t9327, t9329, t9330, t9334, t9336, t9337)
}
