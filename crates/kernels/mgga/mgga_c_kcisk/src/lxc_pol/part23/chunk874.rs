//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 874/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk874<F: Float>(t1391: F, t3278: F, t3532: F, t967: F, t143: F, t3283: F, t443: F, t1390: F, t213: F, t1056: F, t3859: F, t1387: F, t3820: F, t3824: F, t1346: F, t3832: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14088 = t1391 * t3278;
    let t14090 = t967 * t3532;
    let t14091 = t14090 * t3278;
    let t14093 = t143 * t3532;
    let t14096 = t443 * t3283;
    let t14100 = t213 * t1390;
    let t14101 = t14100 * t1056;
    let t14103 = t3859 * t3283;
    let t14107 = t1387 * t1056;
    let t14116 = t1391 * t3820;
    let t14118 = t443 * t3824;
    let t14120 = t1346 * t3832;
    (t14088, t14090, t14091, t14093, t14096, t14100, t14101, t14103, t14107, t14116, t14118, t14120)
}
