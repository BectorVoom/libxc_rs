//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1074/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1074<F: Float>(t6475: F, t8232: F, t1882: F, t26480: F, t26211: F, t46862: F, t26214: F, t8392: F, t487: F, t6454: F, t6531: F, t26237: F, t26041: F, t6480: F, t26217: F, t26353: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t103077 = t8232 * t6475;
    let t103082 = 2.0 / 9.0 * t1882 * t26480;
    let t103083 = t46862 * t26211;
    let t103107 = 2.0 / 27.0 * t8392 * t26214;
    let t103108 = t487 * t6454;
    let t103121 = t8232 * t6531;
    let t103142 = 4.0 / 9.0 * t1882 * t26237;
    let t103163 = t26041 * t487;
    let t103195 = t8232 * t6480;
    let t103198 = 2.0 / 9.0 * t1882 * t26217;
    let t103200 = 2.0 / 27.0 * t8392 * t26353;
    (t103077, t103082, t103083, t103107, t103108, t103121, t103142, t103163, t103195, t103198, t103200)
}
