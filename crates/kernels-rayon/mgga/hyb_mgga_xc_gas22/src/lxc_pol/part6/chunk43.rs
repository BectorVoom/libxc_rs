//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 43/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk43(t43: f64, t47: f64, t51: f64, t54: f64, t57: f64, t60: f64, t63: f64, t66: f64, t69: f64, t72: f64, t88: f64) -> f64 {
    let t44 = 0.135e1_f64 <= t43;
    let t92 = piecewise3(t44, 1.0_f64 / t47 / 36.0_f64 - t51 / 960.0_f64 + t54 / 26880.0_f64 - t57 / 829440.0_f64 + t60 / 28385280.0_f64 - t63 / 0.107347968e10_f64 + t66 / 0.445906944e11_f64 - t69 / 0.20214448128e13_f64, 1.0_f64 - 8.0_f64 / 3.0_f64 * t72 * t88);
    t92
}
