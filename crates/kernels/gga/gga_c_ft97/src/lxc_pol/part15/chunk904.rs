//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 904/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk904<F: Float>(t1771: F, t4455: F, t4459: F, t4527: F, t4519: F, t8282: F, t4512: F, t4523: F, t1636: F, t4496: F, t89: F, t57435: F) -> (F, F, F, F, F, F, F, F) {
    let t59002 = t1771 * t4455;
    let t59007 = t1771 * t4459;
    let t59078 = t1771 * t4527;
    let t59102 = t8282 * t4519;
    let t59104 = t8282 * t4512;
    let t59143 = t8282 * t4523;
    let t59170 = t89 * t1636 * t4496;
    let t59339 = F::new(8.0) / F::new(27.0) * t57435;
    (t59002, t59007, t59078, t59102, t59104, t59143, t59170, t59339)
}
