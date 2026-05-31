//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 578/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk578<F: Float>(t143: F, t7954: F, t9071: F, t24: F, t7368: F, t603: F, t157: F, t161: F, t7943: F, t89: F, t2252: F, t342: F, t657: F) -> (F, F, F, F, F, F, F, F) {
    let t9327 = t7954 * t143;
    let t9383 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t9071;
    let t9432 = t24 * t7368;
    let t9437 = t603 * t603;
    let t9438 = F::cast_from(1.0_f64) / t9437;
    let t9439 = t157 * t9438;
    let t9457 = F::cast_from(28.0_f64) / F::cast_from(81.0_f64) * t89 * t7943 * t161;
    let t9482 = t342 * t2252 * t657 / F::cast_from(18.0_f64);
    (t9327, t9383, t9432, t9437, t9438, t9439, t9457, t9482)
}
