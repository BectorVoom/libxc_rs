//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1165/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1165<F: Float>(t22583: F, t25688: F, t92466: F, t22576: F, t420: F, t383: F, t401: F, t930: F, t358: F, t938: F, t363: F, t428: F, t53: F) -> (F, F, F, F, F, F) {
    let t100554 = t22583 * t92466 * t25688;
    let t100556 = t420 * t22576;
    let t100558 = t930 * t383 * t401;
    let t100580 = t938 * t358;
    let t100581 = t363 * t428;
    let t100586 = t363 * t53;
    (t100554, t100556, t100558, t100580, t100581, t100586)
}
