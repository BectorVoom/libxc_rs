//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 852/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk852<F: Float>(t1049: F, t442: F, t13964: F, t12951: F, t167: F, t1391: F, t3278: F, t3532: F, t967: F, t143: F, t3283: F, t443: F, t1390: F, t213: F, t1056: F, t3859: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14082 = t1049 * t442;
    let t14083 = 0.62154466893555682512e-3 * t14082;
    let t14084 = 0.71734315950379065738e-1 * t13964;
    let t14085 = t167 * t12951;
    let t14088 = t1391 * t3278;
    let t14090 = t967 * t3532;
    let t14091 = t14090 * t3278;
    let t14093 = t143 * t3532;
    let t14096 = t443 * t3283;
    let t14100 = t213 * t1390;
    let t14101 = t14100 * t1056;
    let t14103 = t3859 * t3283;
    (t14083, t14084, t14085, t14088, t14091, t14093, t14096, t14101, t14103)
}
