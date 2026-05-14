//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 286/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk286<F: Float>(t241: F, t2542: F, t258: F, t681: F, t756: F, t89: F, t2399: F, t259: F, t1882: F, t731: F, t768: F, t713: F, t729: F, t773: F, t2459: F, t265: F) -> (F, F, F, F, F, F, F) {
    let t2544 = t241 * t2542 * t258;
    let t2549 = t89 * t681 * t756;
    let t2553 = 4.0 / 27.0 * t89 * t2399 * t259;
    let t2554 = t1882 * t731;
    let t2556 = t1882 * t768;
    let t2559 = t729 * t773 * t713;
    let t2563 = t729 * t265 * t2459;
    (t2544, t2549, t2553, t2554, t2556, t2559, t2563)
}
