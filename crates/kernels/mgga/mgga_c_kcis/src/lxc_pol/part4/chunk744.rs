//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 744/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk744<F: Float>(t4893: F, t4919: F, t1030: F, t1083: F, t1697: F, t1745: F, t278: F, t305: F, t3056: F, t3057: F, t3059: F, t3061: F, t3158: F, t339: F, t4768: F, t4831: F, t4833: F, t4837: F, t4840: F, t4843: F, t4845: F, t4849: F, t4852: F, t975: F) -> (F, F) {
    let t4920 = t4893 + t4919;
    let t4922 = t3056 + 0.23426533963880895498e-2 * t3057 + 0.46853067927761790996e-2 * t3059 + 0.23426533963880895498e-2 * t4831 + 0.46853067927761790996e-2 * t3061 * t4833 + 0.46853067927761790996e-2 * t1030 * t4837 - 0.46853067927761790996e-2 * t3158 * t4840 + 0.46853067927761790996e-2 * t4843 + 0.46853067927761790996e-2 * t1030 * t4845 + 0.14055920378328537299e-1 * t305 * t4849 - 0.46853067927761790996e-2 * t305 * t4852 - t4768 * t339 - t1697 * t1083 - t975 * t1745 - t278 * t4920;
    (t4920, t4922)
}
