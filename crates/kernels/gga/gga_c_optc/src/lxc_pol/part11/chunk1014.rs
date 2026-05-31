//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1014/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1014<F: Float>(t1764: F, t22512: F, t2002: F, t518: F, t517: F, t11: F, t2: F, t209: F, t6567: F, t543: F, t6374: F, t1776: F) -> (F, F, F, F, F, F) {
    let t22513 = t1764 * t22512;
    let t22515 = t518 * t2002;
    let t22516 = t517 * t22515;
    let t22519 = F::powf(t11, -F::cast_from(0.25e1_f64));
    let t22522 = t22519 * t2 * t6567 * t209;
    let t22524 = t6374 * t543;
    let t22526 = t1776 * t22512;
    (t22513, t22515, t22516, t22522, t22524, t22526)
}
