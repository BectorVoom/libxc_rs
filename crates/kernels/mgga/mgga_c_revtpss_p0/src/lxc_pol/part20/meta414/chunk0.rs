//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1529/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1529<F: Float>(t11692: F, t11922: F, t4899: F, t1086: F, t11213: F, t3090: F, t3057: F, t3316: F, t4891: F, t3298: F, t3059: F, t3154: F) -> (F, F, F, F, F) {
    let t43035 = t4899 * t11922 * t11692;
    let t43038 = t11213 * t1086 * t3090;
    let t43043 = t3057 * t3316;
    let t43044 = t43043 * t4891;
    let t43049 = t3057 * t3298;
    let t43050 = t43049 * t4891;
    let t43051 = t3154 * t3059;
    (t43035, t43038, t43044, t43050, t43051)
}
