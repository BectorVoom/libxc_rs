//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2409/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2409<F: Float>(t11200: F, t3286: F, t3046: F, t4995: F, t3057: F, t3143: F, t42859: F, t342: F, t16551: F, t994: F, t16558: F, t16505: F) -> (F, F, F, F, F, F, F, F) {
    let t43446 = t11200 * t3286;
    let t43453 = t3046 * t4995;
    let t43456 = t3057 * t4995;
    let t43471 = t42859 * t3143;
    let t43472 = t342 * t43471;
    let t43520 = t994 * t16551;
    let t43524 = t994 * t16558;
    let t43528 = t994 * t16505;
    (t43446, t43453, t43456, t43471, t43472, t43520, t43524, t43528)
}
