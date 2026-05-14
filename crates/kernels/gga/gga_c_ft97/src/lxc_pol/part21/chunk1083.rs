//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1083/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1083<F: Float>(t1058: F, t5842: F, t1349: F, t26534: F, t376: F, t26788: F, t5766: F, t26539: F, t26545: F, t24087: F, t6580: F, t358: F, t165: F, t7800: F, t1359: F, t3588: F) -> (F, F, F, F, F, F, F, F, F) {
    let t104289 = t5842 * t1058;
    let t104306 = 2.0 / 9.0 * t1349 * t376 * t26534;
    let t104308 = 2.0 / 9.0 * t5766 * t26788;
    let t104311 = 2.0 / 9.0 * t1349 * t376 * t26539;
    let t104314 = 2.0 / 9.0 * t1349 * t376 * t26545;
    let t104316 = t6580 * t24087 / 9.0;
    let t104321 = t1058 * t358;
    let t104331 = t165 * t7800;
    let t104336 = t1359 * t3588;
    (t104289, t104306, t104308, t104311, t104314, t104316, t104321, t104331, t104336)
}
