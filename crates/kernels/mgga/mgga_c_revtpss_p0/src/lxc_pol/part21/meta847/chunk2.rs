//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3177/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3177<F: Float>(t56228: F, t58145: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t58138: F, t58141: F, t58143: F, t58147: F, t43858: F, t43928: F, t58151: F, t58153: F, t58156: F, t58158: F, t58160: F, t58162: F, t58165: F, t58168: F, t58171: F, t58174: F) -> (F, F) {
    let t58536 = F::cast_from(0.39862222222222222223e0_f64) * t56228;
    let t58543 = F::cast_from(0.27385555555555555556e0_f64) * t58145;
    let t58545 = F::cast_from(0.99655555555555555555e0_f64) * t56221 + F::new(0.17938e1) * t56226 + t58536 - F::cast_from(0.29896666666666666667e0_f64) * t56230 + F::cast_from(0.29896666666666666667e0_f64) * t56234 - F::cast_from(0.31003950617283950619e0_f64) * t56236 + F::new(0.1898925e1) * t58138 + F::cast_from(0.427258125e1_f64) * t58141 - F::cast_from(0.230371875e0_f64) * t58143 + t58543 - F::cast_from(0.16431333333333333333e0_f64) * t58147;
    let t58558 = F::cast_from(0.82156666666666666667e-1_f64) * t58151 - F::cast_from(0.24342716049382716049e0_f64) * t58153 + F::cast_from(0.49293999999999999999e0_f64) * t58156 + F::cast_from(0.10954222222222222222e0_f64) * t58158 + F::cast_from(0.54771111111111111111e-1_f64) * t58160 + F::cast_from(0.32862666666666666667e0_f64) * t58162 + F::cast_from(0.54771111111111111111e-1_f64) * t43928 - F::cast_from(0.91285185185185185185e-1_f64) * t58165 + F::cast_from(0.10954222222222222222e0_f64) * t58168 - F::cast_from(0.98587999999999999998e0_f64) * t58171 - F::cast_from(0.85199506172839506175e-1_f64) * t58174 - F::cast_from(0.11072839506172839506e0_f64) * t43858;
    (t58545, t58558)
}
