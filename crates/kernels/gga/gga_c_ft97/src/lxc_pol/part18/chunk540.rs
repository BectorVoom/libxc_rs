//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 540/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk540<F: Float>(t452: F, t488: F, t6478: F, t110: F, t6454: F, t1339: F, t447: F, t925: F, t942: F, t1307: F, t965: F) -> (F, F, F, F, F) {
    let t6480 = t452 * t488 * t6478;
    let t6484 = t452 * t110 * t6454;
    let t6488 = t447 * t1339 * t925;
    let t6492 = t452 * t1339 * t942;
    let t6495 = t1307 * t965;
    (t6480, t6484, t6488, t6492, t6495)
}
