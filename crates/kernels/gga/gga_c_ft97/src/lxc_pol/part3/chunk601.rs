//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 601/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk601<F: Float>(t603: F, t157: F, t161: F, t7943: F, t89: F, t2252: F, t342: F, t657: F, t173: F, t703: F) -> (F, F, F, F) {
    let t9437 = t603 * t603;
    let t9438 = 1.0 / t9437;
    let t9439 = t157 * t9438;
    let t9457 = 28.0 / 81.0 * t89 * t7943 * t161;
    let t9482 = t342 * t2252 * t657 / 18.0;
    let t9483 = t173 * t703;
    (t9439, t9457, t9482, t9483)
}
