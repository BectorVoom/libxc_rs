//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 773/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk773<F: Float>(t12590: F, t13212: F, t3455: F, t379: F, t9144: F, t2179: F, t582: F) -> (F, F, F, F) {
    let t13213 = t13212 * t12590;
    let t13216 = t3455 * t379;
    let t13217 = t9144 * t13216;
    let t13220 = t582 * t2179;
    (t13213, t13216, t13217, t13220)
}
