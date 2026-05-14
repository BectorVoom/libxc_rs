//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1169/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1169<F: Float>(t1626: F, t3011: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11574: F, t15127: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F) {
    let t15350 = t1626 * t3011;
    let t15363 = 0.2283111111111111111e-1 * t15125;
    let t15364 = 0.11415555555555555555e-1 * t15191;
    let t15373 = -t11574 - 0.1522074074074074074e-1 * t11134 + 0.38051851851851851851e-2 * t11136 - 0.11415555555555555555e-1 * t11138 + 0.57077777777777777777e-2 * t11140 - 0.76103703703703703702e-2 * t15189 + 0.76103703703703703701e-2 * t15127 - t15363 + t15364 - 0.19025925925925925925e-1 * t15142 + 0.68493333333333333331e-1 * t15156 - 0.2283111111111111111e-1 * t15132 - 0.11415555555555555555e-1 * t15137 - 0.10274e0 * t15160 + 0.68493333333333333332e-1 * t15147 + 0.34246666666666666666e-1 * t15151 - 0.17123333333333333333e-1 * t15195;
    (t15350, t15373)
}
