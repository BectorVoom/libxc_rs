//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 540/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk540<F: Float>(t1184: F, t3382: F, t1190: F, t1162: F, t3360: F, t1111: F, t1181: F, t3201: F, t1172: F, t2450: F, t1024: F, t134: F) -> (F, F, F, F, F, F, F) {
    let t3383 = t3382 * t1184;
    let t3385 = t3382 * t1190;
    let t3391 = t3360 * t1162;
    let t3393 = t1181 * t3201 * t1111;
    let t3394 = t3391 * t3393;
    let t3396 = t2450 * t1172;
    let t3401 = t134 * t1024;
    (t3383, t3385, t3391, t3393, t3394, t3396, t3401)
}
