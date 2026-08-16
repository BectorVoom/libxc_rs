//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3103/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3103(t56236: f64, t58117: f64, t58134: f64, t68389: f64, t68399: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t43888: f64, t58146: f64, t58153: f64, t58166: f64, t81242: f64, t81245: f64, t81489: f64, t81491: f64, t81494: f64, t81496: f64, t81499: f64, t81501: f64) -> (f64, f64) {
    let t81729 = t58117 + 0.929655e1_f64 * t81224 + 0.516475e0_f64 * t81228 - 0.19128703703703703704e0_f64 * t81230 + 0.68863333333333333333e0_f64 * t81232 - 0.103295e1_f64 * t81234 - 0.17215833333333333333e0_f64 * t81236 + t58134 - 0.16068111111111111111e1_f64 * t56236 - 0.51647499999999999999e0_f64 * t68389 + 0.13772666666666666667e1_f64 * t68399;
    let t81740 = 0.17215833333333333333e1_f64 * t81242 - 0.61977e1_f64 * t81245 - 0.62517e0_f64 * t81489 - 0.41678e0_f64 * t81491 + 0.187551e1_f64 * t81494 - 0.30872592592592592593e-1_f64 * t81496 + 0.794188125e1_f64 * t81499 - 0.473371875e0_f64 * t81501 + t58146 - 0.92617777777777777779e0_f64 * t58153 - t58166 - 0.5356037037037037037e0_f64 * t43888;
    (t81729, t81740)
}
