//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 880/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk880<F: Float>(t38052: F, t38053: F, t92: F, t358: F, t37391: F, t378: F, t11401: F, t23: F, t26: F, t37357: F, t37406: F, t7954: F) -> (F, F, F, F, F, F, F, F) {
    let t38055 = t92 * t38052 * t38053;
    let t38057 = t358 * t37391;
    let t38059 = t92 * t378 * t38057;
    let t38061 = t11401 * t23;
    let t38062 = t26 * t38061;
    let t38063 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t38062;
    let t38064 = t37406 * t37357;
    let t38066 = t92 * t7954 * t38064;
    (t38055, t38057, t38059, t38061, t38062, t38063, t38064, t38066)
}
