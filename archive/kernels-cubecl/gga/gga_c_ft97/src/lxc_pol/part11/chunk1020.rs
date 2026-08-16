//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1020/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1020<F: Float>(t41435: F, t446: F, t9744: F, t1882: F, t9772: F, t10039: F, t684: F, t9770: F, t2346: F, t2359: F) -> (F, F, F, F, F) {
    let t41437 = t446 * t9744 * t41435;
    let t41439 = t1882 * t9772;
    let t41441 = t10039 * t684;
    let t41443 = t446 * t9770 * t41441;
    let t41446 = F::cast_from(1.0_f64) / t2346 / t2359;
    (t41437, t41439, t41441, t41443, t41446)
}
