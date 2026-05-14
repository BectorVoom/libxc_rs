//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1249/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1249<F: Float>(t1349: F, t30161: F, t376: F, t104161: F, t104252: F, t104289: F, t104306: F, t104308: F, t104311: F, t104314: F, t104316: F, t104364: F, t1969: F, t23413: F, t27416: F, t30285: F, t30288: F, t379: F, t5772: F, t925: F, t94227: F, t95009: F) -> (F,) {
    let t119198 = t1349 * t376 * t30161;
    let t119208 = 2.0 / 9.0 * t23413 * t30285 - t5772 * t95009 * t30288 * t379 / 3.0 + 2.0 / 9.0 * t5772 * t104161 * t27416 - t104252 - t119198 / 18.0 - t94227 + t104306 + t104308 + t104311 + t104314 - t104316 - t5772 * t1969 * t104289 * t925 / 9.0 - t5772 * t1969 * t104364 * t925 / 9.0;
    (t119208,)
}
