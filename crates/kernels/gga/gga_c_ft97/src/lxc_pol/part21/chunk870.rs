//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 870/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk870<F: Float>(t609: F, t6718: F, t2179: F, t1053: F, t5968: F, t1384: F, t3565: F, t6708: F, t9276: F, t12664: F, t5956: F, t1017: F, t614: F, t5778: F, t28: F, t165: F, t3408: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26520 = t6718 * t609;
    let t26521 = t2179 * t26520;
    let t26523 = t5968 * t1053;
    let t26524 = t2179 * t26523;
    let t26526 = t1384 * t3565;
    let t26527 = t2179 * t26526;
    let t26529 = t9276 * t6708;
    let t26531 = t12664 * t5956;
    let t26533 = t614 * t1017;
    let t26534 = t5778 * t26533;
    let t26535 = t28 * t26534;
    let t26538 = t165 * t3408;
    (t26520, t26521, t26523, t26524, t26526, t26527, t26529, t26531, t26533, t26534, t26535, t26538)
}
