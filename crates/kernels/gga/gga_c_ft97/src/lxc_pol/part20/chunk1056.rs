//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1056/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1056<F: Float>(t3886: F, t97190: F, t27762: F, t6118: F, t24437: F, t24438: F, t27855: F, t684: F, t2409: F, t6878: F, t6852: F, t24432: F, t27878: F, t27845: F, t13863: F, t24519: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t108033 = t97190 * t3886;
    let t108035 = t6118 * t27762 * t108033;
    let t108039 = t24437 * t24438 * t27855 * t684;
    let t108043 = t24437 * t24438 * t6878 * t2409;
    let t108045 = t6852 * t2409;
    let t108047 = t6118 * t24432 * t108045;
    let t108049 = t27878 * t684;
    let t108051 = t6118 * t24432 * t108049;
    let t108055 = t24437 * t24438 * t27845 * t684;
    let t108057 = t24519 * t13863;
    (t108033, t108035, t108039, t108043, t108045, t108047, t108049, t108051, t108055, t108057)
}
