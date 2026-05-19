//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 603/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk603<F: Float>(t1077: F, t513: F, t1083: F, t398: F, t879: F, t1095: F, t384: F, t1131: F, t506: F, t1441: F, t997: F, t839: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4516 = t513 * t1077;
    let t4518 = t398 * t1083 * t4516;
    let t4521 = t513 * t879;
    let t4523 = t398 * t1095 * t4521;
    let t4524 = t384 * t4523;
    let t4526 = t506 * t1131;
    let t4528 = t398 * t1083 * t4526;
    let t4532 = F::cast_from(0.16006300097412701803e-1_f64) * t997 * t1441;
    let t4533 = t513 * t839;
    (t4516, t4518, t4521, t4523, t4524, t4526, t4528, t4532, t4533)
}
