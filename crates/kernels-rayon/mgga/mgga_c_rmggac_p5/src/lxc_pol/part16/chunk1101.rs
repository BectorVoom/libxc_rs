//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1101/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1101(t35772: f64, t37848: f64, t37849: f64, t37850: f64, t40516: f64, t40558: f64, t40564: f64, t40566: f64, t43466: f64, t43467: f64, t43472: f64, t47062: f64, t47071: f64, t47073: f64, t47078: f64, t47081: f64, t47100: f64, t47108: f64) -> f64 {
    let t48877 = -0.9579393361491046851e0_f64 * t40516 + 0.5107751987195740728e-4_f64 * t47062 - 0.5107751987195740728e-4_f64 * t47071 + 0.5107751987195740728e-4_f64 * t47073 - 0.3192344991997337955e-4_f64 * t47078 - 0.30487649791575028312e-3_f64 * t35772 - t37848 - t37849 + t37850 + 0.49658699875514145967e-4_f64 * t47081 - 0.49658699875514145965e-4_f64 * t40558 + t43466 - t43467 - 0.49658699875514145965e-4_f64 * t40564 + 0.49658699875514145965e-4_f64 * t40566 - t43472 + 0.47896966807455234256e0_f64 * t47100 + 0.35922725105591425692e0_f64 * t47108;
    t48877
}
