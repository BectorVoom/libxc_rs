//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 671/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk671<F: Float>(t39: F, t6363: F, t1765: F, t745: F, t1764: F, t518: F, t622: F, t517: F, t11: F, t2: F, t1776: F, t525: F) -> (F, F, F, F, F, F, F) {
    let t6364 = t6363 * t39;
    let t6366 = t1765 * t745;
    let t6367 = t1764 * t6366;
    let t6369 = t518 * t622;
    let t6370 = t517 * t6369;
    let t6373 = F::new(1.0)/pow_3_2::<f64>(t11);
    let t6374 = t6373 * t2;
    let t6375 = t6374 * t39;
    let t6377 = t1776 * t6366;
    let t6379 = t525 * t6369;
    (t6364, t6367, t6370, t6374, t6375, t6377, t6379)
}
