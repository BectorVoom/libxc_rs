//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 823/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk823<F: Float>(t1636: F, t5300: F, t89: F, t5343: F, t8282: F, t4939: F, t801: F, t230: F, t4977: F, t2440: F, t39976: F, t5249: F, t703: F, t1196: F, t2725: F, t800: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t70141 = t89 * t1636 * t5300;
    let t70142 = 4.0 / 9.0 * t70141;
    let t70231 = t8282 * t5343;
    let t70278 = t4939 * t801;
    let t70290 = t230 * t4977;
    let t70326 = t2440 * t4939;
    let t70354 = 0.59031789687271907073e-3 * t39976 * t5249;
    let t70402 = t703 * t4977;
    let t70462 = t2725 * t1196;
    let t70463 = t800 * t70462;
    (t70141, t70142, t70231, t70278, t70290, t70326, t70354, t70402, t70462, t70463)
}
