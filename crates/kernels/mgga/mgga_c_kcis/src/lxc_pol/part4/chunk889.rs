//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 889/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk889<F: Float>(t2974: F, t659: F, t2331: F, t946: F, t2977: F, t2971: F, t251: F, t2887: F, t1075: F, t237: F, t240: F) -> (F, F, F, F, F, F) {
    let t9702 = t659 * t2974;
    let t9708 = t2331 * t946;
    let t9710 = t659 * t2977;
    let t9712 = t659 * t2971;
    let t9714 = t251 * t2887;
    let t9725 = t237 * t1075 * t240;
    (t9702, t9708, t9710, t9712, t9714, t9725)
}
