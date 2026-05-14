//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 185/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk185<F: Float>(t912: F, t913: F, t587: F, t600: F, t874: F, t568: F, t193: F, t557: F, t574: F, t597: F, t895: F, t902: F, t904: F, t908: F) -> (F, F, F, F, F) {
    let t914 = t912 * t913;
    let t915 = t587 * t914;
    let t917 = t600 * t874;
    let t918 = t568 * t917;
    let t921 = 0.35750489951850426669e0 * t895 * t193 + 0.14896037479937677779e-1 * t902 - 0.35750489951850426669e0 * t557 * t904 - 0.23005755572352449806e1 * t574 * t908 - 0.95857314884801874192e-1 * t915 + 0.23005755572352449806e1 * t597 * t918;
    (t914, t915, t917, t918, t921)
}
