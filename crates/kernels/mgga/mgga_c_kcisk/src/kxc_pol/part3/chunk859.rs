//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 859/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk859<F: Float>(t14176: F, t3482: F, t13959: F, t3800: F, t3734: F, t3739: F, t3732: F, t3764: F, t1415: F, t1411: F, t1404: F, t3783: F, t3787: F, t3508: F, t3791: F, t3513: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t14177 = t3482 * t14176;
    let t14179 = t13959 * t3800;
    let t14181 = t3739 * t3734;
    let t14183 = t3764 * t3732;
    let t14184 = t1415 * t14183;
    let t14185 = t1411 * t14184;
    let t14187 = t1404 * t3783;
    let t14188 = t14187 * sigma0;
    let t14189 = t14188 * t3787;
    let t14190 = t1411 * t14189;
    let t14192 = t3508 * t3791;
    let t14193 = t1411 * t14192;
    let t14195 = t3739 * t3513;
    (t14177, t14179, t14181, t14185, t14187, t14190, t14193, t14195)
}
