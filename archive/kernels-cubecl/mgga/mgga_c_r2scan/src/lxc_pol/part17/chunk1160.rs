//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1160/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1160<F: Float>(t3295: F, t9156: F, t10781: F, t8813: F, t11802: F, t39375: F, t10710: F, t30428: F, t37712: F, t10768: F, t29194: F, t29177: F, t37658: F) -> (F, F, F, F, F, F) {
    let t43090 = t3295 * t9156;
    let t43092 = t10781 * t8813;
    let t43094 = t39375 * t11802;
    let t43097 = t37712 * t10710 * t30428;
    let t43100 = t10768 * t10710 * t29194;
    let t43103 = t37658 * t10710 * t29177;
    (t43090, t43092, t43094, t43097, t43100, t43103)
}
