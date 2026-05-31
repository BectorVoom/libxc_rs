//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3103/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3103<F: Float>(t56236: F, t58117: F, t58134: F, t68389: F, t68399: F, t81224: F, t81228: F, t81230: F, t81232: F, t81234: F, t81236: F, t43888: F, t58146: F, t58153: F, t58166: F, t81242: F, t81245: F, t81489: F, t81491: F, t81494: F, t81496: F, t81499: F, t81501: F) -> (F, F) {
    let t81729 = t58117 + F::cast_from(0.929655e1_f64) * t81224 + F::cast_from(0.516475e0_f64) * t81228 - F::cast_from(0.19128703703703703704e0_f64) * t81230 + F::cast_from(0.68863333333333333333e0_f64) * t81232 - F::cast_from(0.103295e1_f64) * t81234 - F::cast_from(0.17215833333333333333e0_f64) * t81236 + t58134 - F::cast_from(0.16068111111111111111e1_f64) * t56236 - F::cast_from(0.51647499999999999999e0_f64) * t68389 + F::cast_from(0.13772666666666666667e1_f64) * t68399;
    let t81740 = F::cast_from(0.17215833333333333333e1_f64) * t81242 - F::cast_from(0.61977e1_f64) * t81245 - F::cast_from(0.62517e0_f64) * t81489 - F::cast_from(0.41678e0_f64) * t81491 + F::cast_from(0.187551e1_f64) * t81494 - F::cast_from(0.30872592592592592593e-1_f64) * t81496 + F::cast_from(0.794188125e1_f64) * t81499 - F::cast_from(0.473371875e0_f64) * t81501 + t58146 - F::cast_from(0.92617777777777777779e0_f64) * t58153 - t58166 - F::cast_from(0.5356037037037037037e0_f64) * t43888;
    (t81729, t81740)
}
