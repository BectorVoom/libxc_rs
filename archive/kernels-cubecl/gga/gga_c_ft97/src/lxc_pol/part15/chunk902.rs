//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 902/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk902<F: Float>(t37345: F, t4418: F, t89: F, t1636: F, t4437: F, t46256: F, t4432: F, t7773: F, t46320: F, t4515: F, t8282: F, t1771: F, t4531: F) -> (F, F, F, F, F, F, F) {
    let t57527 = t89 * t37345 * t4418;
    let t57620 = t89 * t1636 * t4437;
    let t57627 = F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t46256;
    let t57718 = t89 * t7773 * t4432;
    let t57767 = F::cast_from(56.0_f64) / F::cast_from(243.0_f64) * t46320;
    let t57980 = t8282 * t4515;
    let t58140 = t1771 * t4531;
    (t57527, t57620, t57627, t57718, t57767, t57980, t58140)
}
