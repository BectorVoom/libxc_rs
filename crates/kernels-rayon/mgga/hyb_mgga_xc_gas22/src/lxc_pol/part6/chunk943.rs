//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 943/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk943(t3124: f64, t7884: f64, t2024: f64, t2027: f64, t3288: f64, t6471: f64, t6474: f64, t6477: f64, t6481: f64, t677: f64, t684: f64, t687: f64, t8560: f64, t8562: f64, t8566: f64, t8570: f64, t8575: f64, t8577: f64) -> (f64, f64) {
    let t8579 = t7884 * t3124;
    let t8583 = t6471 / 144.0_f64 - t6474 / 96.0_f64 - t6477 / 192.0_f64 - t6481 / 144.0_f64 - t8560 - t684 * t687 * t8562 / 32.0_f64 - t684 * t687 * t8566 / 64.0_f64 - t2024 * t2027 * t8570 / 48.0_f64 + t8575 / 288.0_f64 + t8577 / 96.0_f64 - 7.0_f64 / 32.0_f64 * t8579 - 3.0_f64 / 32.0_f64 * t677 * t3288;
    (t8579, t8583)
}
