//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1171/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1171<F: Float>(t11229: F, t2201: F, t26929: F, t3177: F, t10513: F, t283: F, t1194: F, t1095: F, t10525: F, t3463: F, t982: F, t3329: F, t7738: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t92398 = t2201 * t11229;
    let t92437 = t3177 * t26929;
    let t92447 = t10513 * t283;
    let t92514 = sigma0 * t1194;
    let t92515 = t1095 * t92514;
    let t92522 = t10525 * t283;
    let t92544 = t3463 * t982;
    let t92564 = t7738 * t3329;
    (t92398, t92437, t92447, t92515, t92522, t92544, t92564)
}
