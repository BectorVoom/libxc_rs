//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 781/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk781<F: Float>(t2662: F, t7448: F, t2672: F, t769: F, t123: F, t549: F, t7451: F, t2441: F, t2477: F, t2471: F, t827: F) -> (F, F, F, F, F, F, F) {
    let t7491 = t2662 * t7448;
    let t7492 = t2672 * t769;
    let t7493 = t549 * t123;
    let t7494 = t7492 * t7493;
    let t7495 = t7451 * t7494;
    let t7499 = F::cast_from(0.51947267698127589899e2_f64) * t2441 * t2477;
    let t7501 = F::new(1.0) / t2471 / t827;
    (t7491, t7492, t7493, t7494, t7495, t7499, t7501)
}
