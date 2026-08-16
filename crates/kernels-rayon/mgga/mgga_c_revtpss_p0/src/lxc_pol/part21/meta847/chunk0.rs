//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3175/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3175(t43828: f64, t43830: f64, t43832: f64, t43911: f64, t56174: f64, t56176: f64, t56181: f64, t58055: f64, t58057: f64, t58060: f64, t58063: f64, t58107: f64) -> f64 {
    let t58518 = 0.46074375e0_f64 * t58055 + 0.15358125e0_f64 * t58057 - 0.3560484375e1_f64 * t58060 + 0.1151859375e0_f64 * t58063 + 0.3071625e0_f64 * t58107 - 0.32862666666666666666e0_f64 * t43828 - 0.59793333333333333333e0_f64 * t43830 + 0.19931111111111111112e0_f64 * t43832 - 0.91285185185185185185e-1_f64 * t43911 - 0.88582716049382716048e0_f64 * t56174 - 0.26574814814814814816e0_f64 * t56176 + 0.39862222222222222223e1_f64 * t56181;
    t58518
}
