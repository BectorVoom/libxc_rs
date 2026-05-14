//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 938/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk938<F: Float>(t11: F, t2: F, t209: F, t6567: F, t543: F, t6374: F, t1776: F, t22512: F, t22515: F, t525: F, t10195: F, t10345: F, t22508: F, t22510: F, t22513: F, t22516: F) -> (F, F, F, F, F) {
    let t22519 = f64::powf(t11, -0.25e1);
    let t22522 = t22519 * t2 * t6567 * t209;
    let t22524 = t6374 * t543;
    let t22526 = t1776 * t22512;
    let t22528 = t525 * t22515;
    let t22531 = -0.28769444444444444444e1 * t22508 + 0.27618666666666666667e2 * t22510 - 0.10229135802469135803e2 * t22513 + 0.89504938271604938273e1 * t22516 + 0.31310740740740740741e1 * t10195 + 0.366775e-1 * t22522 - 0.58684e0 * t22524 + 0.65204444444444444445e0 * t22526 + 0.5705388888888888889e0 * t22528 + 0.13490888888888888889e1 * t10345;
    (t22522, t22524, t22526, t22528, t22531)
}
