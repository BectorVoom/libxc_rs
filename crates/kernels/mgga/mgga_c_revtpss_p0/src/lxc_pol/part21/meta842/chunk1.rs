//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3155/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3155<F: Float>(t58165: F, t12254: F, t141: F, t56219: F, t3417: F, t56149: F, t43764: F, t56172: F, t43858: F, t43928: F, t58151: F, t58153: F, t58156: F, t58158: F, t58160: F, t58162: F) -> (F, F, F, F) {
    let t58166 = F::cast_from(0.11577222222222222222e0_f64) * t58165;
    let t58168 = t141 * t12254 * t56219;
    let t58171 = t141 * t3417 * t56149;
    let t58174 = t141 * t43764 * t56172;
    let t58177 = F::new(0.104195e0) * t58151 - F::cast_from(0.30872592592592592592e0_f64) * t58153 + F::new(0.62517e0) * t58156 + F::cast_from(0.13892666666666666667e0_f64) * t58158 + F::cast_from(0.69463333333333333334e-1_f64) * t58160 + F::cast_from(0.41678000000000000001e0_f64) * t58162 + F::cast_from(0.69463333333333333332e-1_f64) * t43928 - t58166 + F::cast_from(0.13892666666666666667e0_f64) * t58168 - F::new(0.125034e1) * t58171 - F::cast_from(0.10805407407407407407e0_f64) * t58174 - F::cast_from(0.19128703703703703703e0_f64) * t43858;
    (t58168, t58171, t58174, t58177)
}
