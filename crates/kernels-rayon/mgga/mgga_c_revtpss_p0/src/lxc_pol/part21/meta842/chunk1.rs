//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3155/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3155(t58165: f64, t12254: f64, t141: f64, t56219: f64, t3417: f64, t56149: f64, t43764: f64, t56172: f64, t43858: f64, t43928: f64, t58151: f64, t58153: f64, t58156: f64, t58158: f64, t58160: f64, t58162: f64) -> (f64, f64, f64, f64) {
    let t58166 = 0.11577222222222222222e0_f64 * t58165;
    let t58168 = t141 * t12254 * t56219;
    let t58171 = t141 * t3417 * t56149;
    let t58174 = t141 * t43764 * t56172;
    let t58177 = 0.104195e0_f64 * t58151 - 0.30872592592592592592e0_f64 * t58153 + 0.62517e0_f64 * t58156 + 0.13892666666666666667e0_f64 * t58158 + 0.69463333333333333334e-1_f64 * t58160 + 0.41678000000000000001e0_f64 * t58162 + 0.69463333333333333332e-1_f64 * t43928 - t58166 + 0.13892666666666666667e0_f64 * t58168 - 0.125034e1_f64 * t58171 - 0.10805407407407407407e0_f64 * t58174 - 0.19128703703703703703e0_f64 * t43858;
    (t58168, t58171, t58174, t58177)
}
