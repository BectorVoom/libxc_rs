//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 520/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk520<F: Float>(t3765: F, t409: F, t1004: F, t996: F, t390: F, t3055: F, t383: F, t1039: F, t1032: F, t993: F, t1103: F, t175: F, t3044: F, t398: F, t1036: F, t301: F, t879: F) -> (F, F, F, F, F, F, F, F) {
    let t3766 = t3765 * t409;
    let t3770 = t1004 * t996;
    let t3772 = 0.60023625365297631762e-2 * t3770 * t390;
    let t3775 = t3055 * t383;
    let t3777 = 0.12862205435420921092e-2 * t3775 * t1039;
    let t3782 = 0.30011812682648815881e-2 * t1032 * t993;
    let t3793 = t1032 * t1103;
    let t3806 = t398 * t175 * t3044;
    let t3808 = 0.12862205435420921092e-2 * t1036 * t3806;
    let t3809 = t879 * t301;
    (t3766, t3772, t3777, t3782, t3793, t3806, t3808, t3809)
}
