//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 998/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk998<F: Float>(t1089: F, t4643: F, t598: F, t7533: F, t33953: F, t5275: F, t13287: F, t31195: F, t2299: F, t7630: F, t1413: F, t7712: F, t2310: F, t2001: F, t4728: F, t5270: F) -> (F, F, F, F, F, F, F, F) {
    let t36320 = t598 * t1089 * t4643 * t7533;
    let t36323 = t33953 * t5275;
    let t36325 = t31195 * t13287 * t36323;
    let t36327 = t7630 * t2299;
    let t36331 = t7712 * t1413;
    let t36333 = t7630 * t2310;
    let t36335 = t2001 * t4728;
    let t36344 = t31195 * t13287 * t33953 * t5270;
    (t36320, t36323, t36325, t36327, t36331, t36333, t36335, t36344)
}
