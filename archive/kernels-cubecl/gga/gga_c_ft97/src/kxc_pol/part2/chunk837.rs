//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 837/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk837<F: Float>(t13135: F, t605: F, t144: F, t157: F, t526: F, t1053: F, t2178: F, t2190: F, t160: F, t3408: F, t379: F, t2221: F) -> (F, F, F, F) {
    let t13136 = t605 * t13135;
    let t13137 = t144 * t13136;
    let t13140 = t526 * t157;
    let t13141 = t2178 * t1053;
    let t13142 = t13141 * t2190;
    let t13143 = t13140 * t13142;
    let t13146 = t160 * t3408;
    let t13147 = t13146 * t379;
    let t13148 = t2221 * t13147;
    (t13136, t13137, t13143, t13148)
}
