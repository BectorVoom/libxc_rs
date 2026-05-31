//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 910/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk910<F: Float>(t4664: F, t7773: F, t89: F, t37345: F, t4652: F, t4660: F, t61462: F, t62134: F, t4759: F, t8282: F, t4765: F, t4762: F) -> (F, F, F, F, F, F, F, F) {
    let t62287 = t89 * t7773 * t4664;
    let t62309 = t89 * t37345 * t4652;
    let t62317 = t89 * t7773 * t4660;
    let t62364 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t61462;
    let t62410 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t62134;
    let t62587 = t8282 * t4759;
    let t62599 = t8282 * t4765;
    let t62629 = t8282 * t4762;
    (t62287, t62309, t62317, t62364, t62410, t62587, t62599, t62629)
}
