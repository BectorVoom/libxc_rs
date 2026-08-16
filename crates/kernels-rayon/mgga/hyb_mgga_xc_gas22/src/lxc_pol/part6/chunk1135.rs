//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1135/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1135(t4475: f64, t483: f64, t1112: f64, t11217: f64, t495: f64, t1100: f64, t462: f64, t7426: f64, t7438: f64, t7446: f64, t7452: f64, t7456: f64, t7459: f64, t7463: f64, t7466: f64, t7503: f64, t7506: f64, t7518: f64, t7522: f64, t7523: f64) -> (f64, f64, f64, f64) {
    let t11232 = t4475 * t483;
    let t11233 = t11232 * t1112;
    let t11235 = t11217 * t495;
    let t11237 = t4475 * t1100;
    let t11238 = t462 * t11237;
    let t11241 = -0.5848223622634646207e0_f64 * t11233 + t7426 + t462 * t11235 + t11238 - t7503 + 20.0_f64 * t7506 + t7438 + t7446 - t7452 + t7456 - t7459 - t7463 - t7466 - t7518 - t7522 + 0.24415263074675393405e-3_f64 * t7523;
    (t11232, t11235, t11237, t11241)
}
