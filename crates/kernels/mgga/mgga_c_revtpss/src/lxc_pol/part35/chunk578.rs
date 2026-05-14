//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 578/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk578<F: Float>(t1169: F, t6502: F, t3479: F, t6486: F, t3483: F, t5044: F, t6423: F, t6427: F, t6431: F, t448: F, t1756: F) -> (F, F, F, F, F) {
    let t6503 = t6502 * t1169;
    let t6506 = t6486 * t3479;
    let t6513 = t3483 - 0.61805555555555555556e-2 * t5044 - 0.61805555555555555555e-2 * t6423 + 0.18541666666666666667e-1 * t6427 + 0.92708333333333333333e-2 * t6431;
    let t6514 = t6513 * t448;
    let t6518 = t1756 * t1756;
    (t6503, t6506, t6513, t6514, t6518)
}
