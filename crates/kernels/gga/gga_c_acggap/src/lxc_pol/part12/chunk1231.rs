//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1231/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1231<F: Float>(t29979: F, t36417: F, t638: F, t2132: F, t322: F, t7896: F, t9431: F, t119: F, t9367: F, t2395: F, t30005: F, t8081: F, t8998: F) -> (F, F, F, F, F) {
    let t38181 = t29979 * t638 * t36417;
    let t38185 = t7896 * t2132 * t9431 * t322;
    let t38187 = t119 * t9367;
    let t38190 = t30005 * t2395;
    let t38194 = F::cast_from(0.34694512752820797848e1_f64) * t8998 * t8081;
    (t38181, t38185, t38187, t38190, t38194)
}
