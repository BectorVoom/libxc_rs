//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 860/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk860<F: Float>(t23323: F, t3189: F, t1326: F, t1780: F, t3195: F, t23327: F, t3205: F, t103: F, t6454: F, t379: F, t1902: F, t6466: F, t8392: F, t1901: F, t26319: F, t26322: F, t26326: F, t26330: F, t26334: F, t26337: F, t26340: F, t26343: F, t3281: F, t446: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26346 = t23323 * t3189;
    let t26349 = t1780 * t1326;
    let t26350 = t26349 * t3195;
    let t26353 = t23327 * t3205;
    let t26356 = t103 * t6454;
    let t26357 = t26356 * t379;
    let t26358 = t1902 * t26357;
    let t26361 = t8392 * t6466;
    let t26363 = -t1901 * t26319 / 9.0 - 2.0 / 9.0 * t1901 * t26322 + 2.0 / 3.0 * t446 * t26326 - 2.0 / 9.0 * t3281 * t26330 - t446 * t26334 / 9.0 + 2.0 / 3.0 * t446 * t26337 - t446 * t26340 / 3.0 + t1901 * t26343 / 9.0 + 2.0 / 9.0 * t1901 * t26346 - 2.0 / 27.0 * t1901 * t26350 + t1901 * t26353 / 9.0 + t1901 * t26358 / 9.0 - t26361 / 27.0;
    (t26346, t26349, t26350, t26353, t26356, t26357, t26358, t26361, t26363)
}
