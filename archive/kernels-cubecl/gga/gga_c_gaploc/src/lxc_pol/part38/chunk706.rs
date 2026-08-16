//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 706/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk706<F: Float>(t13492: F, t2508: F, t3616: F, t954: F, t2936: F, t3451: F, t13183: F, t11627: F, t7296: F, t883: F, t943: F, t11603: F, t935: F) -> (F, F, F, F, F, F, F, F) {
    let t13494 = F::cast_from(0.92286314761706691403e-1_f64) * t2508 * t13492;
    let t13495 = t954 * t3616;
    let t13497 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t13495;
    let t13498 = t2936 * t3451;
    let t13501 = F::cast_from(0.1281754371690370714e-2_f64) * t13183;
    let t13503 = t7296 * t883 * t11627;
    let t13504 = t943 * t13503;
    let t13506 = t11603 * t935;
    (t13494, t13495, t13497, t13498, t13501, t13503, t13504, t13506)
}
