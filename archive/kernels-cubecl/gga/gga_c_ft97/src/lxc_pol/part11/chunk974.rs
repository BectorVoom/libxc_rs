//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 974/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk974<F: Float>(t2087: F, t40280: F, t91: F, t1986: F, t2075: F, t28: F, t7368: F, t89: F, t356: F, t37391: F, t519: F, t143: F, t37406: F) -> (F, F, F, F) {
    let t40281 = t2087 * t2087;
    let t40283 = t91 * t40280 * t40281;
    let t40288 = t89 * t28 * t7368 * t1986 * t2075;
    let t40292 = t89 * t356 * t519 * t37391;
    let t40294 = t143 * t37406;
    (t40283, t40288, t40292, t40294)
}
