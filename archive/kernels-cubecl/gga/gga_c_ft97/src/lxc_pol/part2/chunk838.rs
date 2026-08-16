//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 838/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk838<F: Float>(t3421: F, t8392: F, t1045: F, t2101: F, t2224: F, t3446: F, t9419: F, t3565: F, t604: F, t379: F, t2210: F, t2178: F, t358: F) -> (F, F, F, F, F) {
    let t13152 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t3421;
    let t13153 = t2101 * t1045;
    let t13154 = t13153 * t2224;
    let t13157 = t9419 * t3446;
    let t13160 = t604 * t3565;
    let t13161 = t13160 * t379;
    let t13162 = t2210 * t13161;
    let t13165 = t2178 * t358;
    (t13152, t13154, t13157, t13162, t13165)
}
