//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2455/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2455<F: Float>(t21456: F, t28565: F, t343: F, t4540: F, t4546: F, t48329: F, t48336: F, t48339: F, t48374: F, t48379: F, t48382: F, t48397: F, t61447: F, t61472: F, t61489: F, t61495: F, t61557: F, t61597: F, t61600: F, t61602: F, t973: F, t984: F) -> F {
    let t69837 = t48329 + F::cast_from(0.27777777777777777777e-3_f64) * t61447 - F::cast_from(0.9259259259259259259e-3_f64) * t48336 - t48339 - F::cast_from(0.83333333333333333331e-3_f64) * t61472 + F::cast_from(0.37037037037037037036e-3_f64) * t61489 - F::cast_from(0.55555555555555555554e-3_f64) * t61495 - F::cast_from(0.55555555555555555554e-3_f64) * t61557 + t48374 - t48379 + t48382 - F::cast_from(0.25e-2_f64) * t973 * t4546 * t28565 * t4540 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t4546 * t21456 * t984 * t343 - F::cast_from(0.18518518518518518518e-3_f64) * t61597 - F::cast_from(0.24691358024691358024e-3_f64) * t61600 + F::cast_from(0.14814814814814814814e-2_f64) * t61602 + F::cast_from(0.3086419753086419753e-3_f64) * t48397;
    t69837
}
