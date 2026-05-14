//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 684/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk684<F: Float>(t8020: F, t395: F, t3953: F, t7706: F, t3952: F, t2075: F, t2168: F, t3937: F, t3942: F, t1312: F, t1313: F, t7710: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8021 = t8020 * sigma0;
    let t8022 = t8021 * t395;
    let t8032 = t3953 * t7706;
    let t8033 = t3952 * t8032;
    let t8036 = t2075 * t2168;
    let t8037 = t3937 * t8036;
    let t8040 = t3942 * t7706;
    let t8041 = t1312 * t8040;
    let t8044 = t1313 * t7710;
    let t8045 = t1312 * t8044;
    let t8048 = t2168 * t2168;
    (t8021, t8022, t8032, t8033, t8036, t8037, t8040, t8041, t8044, t8045, t8048)
}
