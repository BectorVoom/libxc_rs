//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1273/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1273<F: Float>(t119558: F, t3281: F, t9049: F, t27034: F, t3424: F, t446: F, t9073: F, t119571: F, t105797: F, t16666: F, t1901: F, t17189: F, t5916: F, t119567: F, t4668: F, t5842: F) -> (F, F, F, F, F, F, F, F, F) {
    let t119766 = t3281 * t9049 * t119558;
    let t119768 = t27034 * t3424;
    let t119770 = t446 * t9073 * t119768;
    let t119773 = t446 * t9073 * t119571;
    let t119776 = t1901 * t105797 * t16666;
    let t119778 = t5916 * t17189;
    let t119780 = t446 * t9073 * t119778;
    let t119783 = t3281 * t9073 * t119567;
    let t119785 = t5842 * t4668;
    (t119766, t119768, t119770, t119773, t119776, t119778, t119780, t119783, t119785)
}
