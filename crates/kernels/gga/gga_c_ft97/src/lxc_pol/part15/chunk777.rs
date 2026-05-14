//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 777/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk777<F: Float>(t487: F, t7800: F, t179: F, t37406: F, t70: F, t8119: F, t37355: F, t8639: F, t41: F, t1736: F, t639: F, t2281: F, t422: F, t71: F, t8618: F, t118: F, t37993: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39345 = t487 * t7800;
    let t39417 = t179 * t37406;
    let t39430 = t70 * t8119;
    let t39431 = t179 * t37355;
    let t39447 = t8639 * t70;
    let t39448 = t41 * t39447;
    let t39487 = t1736 * t639;
    let t39495 = t422 * t2281;
    let t39514 = t71 * t8618;
    let t39538 = 1.0 / t118 / t37993;
    (t39345, t39417, t39430, t39431, t39448, t39487, t39495, t39514, t39538)
}
