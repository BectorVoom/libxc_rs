//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 830/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk830<F: Float>(t294: F, t7639: F, t13: F, t21: F, t2: F, t7242: F, t113: F, t7806: F, t8494: F, t446: F, t7793: F, t376: F, t7756: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t33828 = F::cast_from(1.0_f64) / t7639 / t294;
    let t36377 = t13 * t21;
    let t36452 = t7242 * t2;
    let t36827 = t13 * t113;
    let t37252 = t7806 * t8494;
    let t37254 = t446 * t7793 * t37252;
    let t37257 = t89 * t376 * t7756;
    (t33828, t36377, t36452, t36827, t37252, t37254, t37257)
}
