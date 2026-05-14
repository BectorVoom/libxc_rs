//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1335/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1335<F: Float>(t117687: F, t33196: F, t117633: F, t9740: F, t116210: F, t34559: F, t964: F, t33258: F, t34484: F, t10004: F, t33257: F, t10000: F, t33234: F, t33162: F, t10005: F, t123: F, t2801: F, t34578: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t117715 = 0.40208333333333333334e-2 * t33196 * t117687;
    let t117729 = 0.34722222222222222222e-2 * t9740 * t117633;
    let t117730 = t9740 * t117687;
    let t117739 = 0.15476481481481481481e-2 * t116210;
    let t117740 = t964 * t34559;
    let t117751 = 0.13402777777777777778e-2 * t33258 * t34484;
    let t117752 = t33257 * t10004;
    let t117764 = 0.34722222222222222222e-2 * t10000 * t33234;
    let t117767 = 0.34722222222222222222e-2 * t10000 * t33162;
    let t117773 = t10005 * t33162;
    let t117784 = t2801 * t34578 * t123;
    (t117715, t117729, t117730, t117739, t117740, t117751, t117752, t117764, t117767, t117773, t117784)
}
