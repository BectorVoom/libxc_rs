//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2839/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2839<F: Float>(t1041: F, t1046: F, t42994: F, t1086: F, t11213: F, t3090: F, t3057: F, t3316: F, t4891: F, t3298: F, t3059: F, t3154: F) -> (F, F, F, F, F, F, F) {
    let t42996 = t1041 * t42994 * t1046;
    let t43038 = t11213 * t1086 * t3090;
    let t43043 = t3057 * t3316;
    let t43044 = t43043 * t4891;
    let t43049 = t3057 * t3298;
    let t43050 = t43049 * t4891;
    let t43051 = t3154 * t3059;
    (t42996, t43038, t43043, t43044, t43049, t43050, t43051)
}
