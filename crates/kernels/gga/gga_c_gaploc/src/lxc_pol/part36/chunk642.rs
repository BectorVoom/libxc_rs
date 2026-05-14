//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 642/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk642<F: Float>(t13119: F, t10893: F, t959: F, t13079: F, t13098: F, t13102: F, t13106: F, t13110: F, t13113: F, t13114: F, t13115: F, t13116: F, t13117: F, t317: F, t797: F, t813: F, t833: F) -> (F,) {
    let t13120 = 0.59584149919750711116e-1 * t13119;
    let t13121 = t10893 * t959;
    let t13123 = t13079 + 0.35750489951850426669e0 * t13098 * t317 - 0.35750489951850426669e0 * t797 * t13102 - 0.23005755572352449806e1 * t813 * t13106 + 0.23005755572352449806e1 * t833 * t13110 - t13113 - t13114 + t13115 + t13116 + t13117 + t13120 + 0.29792074959875355558e-1 * t13121;
    (t13123,)
}
