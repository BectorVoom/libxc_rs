//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3167/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3167<F: Float>(t56228: F, t58145: F, t56221: F, t56226: F, t56230: F, t56234: F, t56236: F, t58138: F, t58141: F, t58143: F, t58147: F, t43858: F, t43928: F, t58151: F, t58153: F, t58156: F, t58158: F, t58160: F, t58162: F, t58165: F, t58168: F, t58171: F, t58174: F) -> (F, F) {
    let t58404 = F::cast_from(0.40256666666666666668e0_f64) * t56228;
    let t58411 = F::cast_from(0.27595e0_f64) * t58145;
    let t58413 = F::cast_from(0.10064166666666666666e1_f64) * t56221 + F::cast_from(0.181155e1_f64) * t56226 + t58404 - F::cast_from(0.30192500000000000001e0_f64) * t56230 + F::cast_from(0.301925e0_f64) * t56234 - F::cast_from(0.31310740740740740741e0_f64) * t56236 + F::cast_from(0.258925e1_f64) * t58138 + F::cast_from(0.58258125e1_f64) * t58141 - F::cast_from(0.1237865625e0_f64) * t58143 + t58411 - F::cast_from(0.16557e0_f64) * t58147;
    let t58426 = F::cast_from(0.82785e-1_f64) * t58151 - F::cast_from(0.24528888888888888889e0_f64) * t58153 + F::cast_from(0.49671e0_f64) * t58156 + F::cast_from(0.11038e0_f64) * t58158 + F::cast_from(0.55190000000000000001e-1_f64) * t58160 + F::cast_from(0.33114000000000000001e0_f64) * t58162 + F::cast_from(0.55190000000000000001e-1_f64) * t43928 - F::cast_from(0.91983333333333333334e-1_f64) * t58165 + F::cast_from(0.11038e0_f64) * t58168 - F::cast_from(0.99342e0_f64) * t58171 - F::cast_from(0.8585111111111111111e-1_f64) * t58174 - F::cast_from(0.11182407407407407408e0_f64) * t43858;
    (t58413, t58426)
}
