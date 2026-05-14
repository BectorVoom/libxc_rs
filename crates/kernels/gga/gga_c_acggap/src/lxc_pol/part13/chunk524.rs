//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 524/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk524<F: Float>(t1184: F, t3409: F, t1190: F, t1165: F, t407: F, t991: F, t1163: F, t1171: F, t3370: F, t1170: F, t1177: F, t174: F, t929: F, t1539: F, t932: F, t952: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3410 = t3409 * t1184;
    let t3412 = t3409 * t1190;
    let t3427 = t1165 * t991 * t407;
    let t3428 = t1163 * t3427;
    let t3430 = t3370 * t1171;
    let t3431 = t1170 * t3430;
    let t3432 = t3431 * t1177;
    let t3439 = t174 * t929;
    let t3445 = t1165 * t3439 * t1539;
    let t3446 = t1163 * t3445;
    let t3449 = t952 * t932;
    (t3410, t3412, t3427, t3428, t3431, t3432, t3445, t3446, t3449)
}
