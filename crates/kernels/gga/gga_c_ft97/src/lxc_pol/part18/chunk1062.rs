//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1062/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1062<F: Float>(t11801: F, t487: F, t8417: F, t971: F, t1851: F, t3170: F, t1786: F, t11401: F, t443: F, t444: F, t110: F, t38477: F, t38463: F, t1587: F, t480: F, t463: F, t8466: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46556 = t11801 * t487;
    let t46565 = t971 * t8417;
    let t46727 = t3170 * t1851;
    let t46809 = t1786 * t3170;
    let t46862 = t443 * t444 * t11401;
    let t46874 = t38477 * t110;
    let t46881 = t38463 * t110;
    let t47007 = t1587 * t480;
    let t47089 = t463 * t8466;
    (t46556, t46565, t46727, t46809, t46862, t46874, t46881, t47007, t47089)
}
