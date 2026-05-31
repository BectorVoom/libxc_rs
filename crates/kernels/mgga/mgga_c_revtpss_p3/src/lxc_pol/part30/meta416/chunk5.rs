//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1571/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1571<F: Float>(t15494: F, t324: F, t15125: F, t15191: F, t11134: F, t11136: F, t11138: F, t11140: F, t11534: F, t15127: F, t15132: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F, t15189: F, t15195: F) -> (F, F) {
    let t15495 = t15494 * t324;
    let t15503 = F::cast_from(0.23744444444444444444e-1_f64) * t15125;
    let t15504 = F::cast_from(0.11872222222222222222e-1_f64) * t15191;
    let t15513 = -t11534 - F::cast_from(0.15829629629629629629e-1_f64) * t11134 + F::cast_from(0.39574074074074074073e-2_f64) * t11136 - F::cast_from(0.11872222222222222222e-1_f64) * t11138 + F::cast_from(0.5936111111111111111e-2_f64) * t11140 - F::cast_from(0.79148148148148148146e-2_f64) * t15189 + F::cast_from(0.79148148148148148146e-2_f64) * t15127 - t15503 + t15504 - F::cast_from(0.19787037037037037037e-1_f64) * t15142 + F::cast_from(0.71233333333333333332e-1_f64) * t15156 - F::cast_from(0.23744444444444444444e-1_f64) * t15132 - F::cast_from(0.11872222222222222222e-1_f64) * t15137 - F::cast_from(0.10685e0_f64) * t15160 + F::cast_from(0.71233333333333333332e-1_f64) * t15147 + F::cast_from(0.35616666666666666666e-1_f64) * t15151 - F::cast_from(0.17808333333333333333e-1_f64) * t15195;
    (t15495, t15513)
}
