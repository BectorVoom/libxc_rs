//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1096/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1096<F: Float>(t2595: F, t38892: F, t12272: F, t7324: F, t41574: F, t41575: F, t42906: F, t46846: F, t46847: F, t46848: F, t47063: F, t47080: F, t47083: F) -> F {
    let t47085 = t38892 * t2595;
    let t47087 = t7324 * t12272;
    let t47089 = -F::cast_from(6.0_f64) * t47080 + t46846 + F::cast_from(2.0_f64) * t47083 + t42906 + t46847 - t46848 + F::cast_from(2.0_f64) * t47085 + F::cast_from(2.0_f64) * t47087 + t41574 + t41575 - t47063;
    t47089
}
