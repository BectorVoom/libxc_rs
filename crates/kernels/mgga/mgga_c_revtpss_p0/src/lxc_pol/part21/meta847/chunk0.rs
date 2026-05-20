//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3175/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3175<F: Float>(t43828: F, t43830: F, t43832: F, t43911: F, t56174: F, t56176: F, t56181: F, t58055: F, t58057: F, t58060: F, t58063: F, t58107: F) -> F {
    let t58518 = F::new(0.46074375e0) * t58055 + F::new(0.15358125e0) * t58057 - F::cast_from(0.3560484375e1_f64) * t58060 + F::cast_from(0.1151859375e0_f64) * t58063 + F::new(0.3071625e0) * t58107 - F::cast_from(0.32862666666666666666e0_f64) * t43828 - F::cast_from(0.59793333333333333333e0_f64) * t43830 + F::cast_from(0.19931111111111111112e0_f64) * t43832 - F::cast_from(0.91285185185185185185e-1_f64) * t43911 - F::cast_from(0.88582716049382716048e0_f64) * t56174 - F::cast_from(0.26574814814814814816e0_f64) * t56176 + F::cast_from(0.39862222222222222223e1_f64) * t56181;
    t58518
}
