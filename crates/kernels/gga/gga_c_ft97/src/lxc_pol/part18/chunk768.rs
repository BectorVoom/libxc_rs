//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 768/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk768<F: Float>(t1053: F, t2178: F, t2190: F, t13140: F, t160: F, t3408: F, t379: F, t2221: F, t3421: F, t8392: F, t1045: F, t2101: F, t2224: F, t3446: F, t9419: F, t3565: F, t604: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13141 = t2178 * t1053;
    let t13142 = t13141 * t2190;
    let t13143 = t13140 * t13142;
    let t13146 = t160 * t3408;
    let t13147 = t13146 * t379;
    let t13148 = t2221 * t13147;
    let t13152 = 2.0 / 27.0 * t8392 * t3421;
    let t13153 = t2101 * t1045;
    let t13154 = t13153 * t2224;
    let t13157 = t9419 * t3446;
    let t13160 = t604 * t3565;
    (t13142, t13143, t13147, t13148, t13152, t13153, t13154, t13157, t13160)
}
