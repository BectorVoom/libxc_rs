//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 662/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk662<F: Float>(t2101: F, t597: F, t24: F, t7368: F, t603: F, t157: F, t161: F, t7943: F, t89: F, t2252: F, t342: F, t657: F) -> (F, F, F, F, F) {
    let t9419 = t2101 * t597;
    let t9432 = t24 * t7368;
    let t9437 = t603 * t603;
    let t9438 = F::new(1.0) / t9437;
    let t9439 = t157 * t9438;
    let t9457 = F::new(28.0) / F::new(81.0) * t89 * t7943 * t161;
    let t9482 = t342 * t2252 * t657 / F::new(18.0);
    (t9419, t9432, t9439, t9457, t9482)
}
