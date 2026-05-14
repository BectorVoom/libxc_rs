//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 845/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk845<F: Float>(t8656: F, t8675: F, t8682: F, t1736: F, t639: F, t2281: F, t422: F, t71: F, t8618: F, t2284: F, t8640: F, t2007: F, t37627: F, t38111: F, t528: F, t118: F, t37993: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39483 = t8675 * t8656;
    let t39485 = t8675 * t8682;
    let t39487 = t1736 * t639;
    let t39495 = t422 * t2281;
    let t39514 = t71 * t8618;
    let t39524 = t8640 * t2284;
    let t39533 = t2007 * t37627;
    let t39535 = t528 * t38111;
    let t39538 = 1.0 / t118 / t37993;
    (t39483, t39485, t39487, t39495, t39514, t39524, t39533, t39535, t39538)
}
