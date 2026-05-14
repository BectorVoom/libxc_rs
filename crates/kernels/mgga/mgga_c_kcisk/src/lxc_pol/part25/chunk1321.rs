//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1321/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1321<F: Float>(t32990: F, t34097: F, t17182: F, t34260: F, t9664: F, t654: F, t7409: F, t1799: F, t9680: F, t34173: F, t415: F, t4798: F, t10500: F, t15947: F, t5180: F, t716: F) -> (F, F, F, F, F) {
    let t117062 = 0.69444444444444444446e-2 * t32990 * t34097;
    let t117065 = 0.69444444444444444446e-2 * t9664 * t17182 * t34260;
    let t117066 = t7409 * t654;
    let t117068 = t1799 * t117066 * t9680;
    let t117074 = t415 * t34173 * t4798;
    let t117078 = t10500 * t5180 * t716 * t15947;
    (t117062, t117065, t117068, t117074, t117078)
}
