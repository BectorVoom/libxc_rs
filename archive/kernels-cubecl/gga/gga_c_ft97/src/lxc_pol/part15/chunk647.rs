//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 647/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk647<F: Float>(t2253: F, t4869: F, t4857: F, t4872: F, t8618: F, t4861: F, t8675: F, t4874: F, t4885: F, t1073: F, t920: F, t1526: F, t4906: F, t9483: F) -> (F, F, F, F, F, F, F, F) {
    let t17554 = t2253 * t4869;
    let t17556 = t2253 * t4857;
    let t17567 = t8618 * t4872;
    let t17573 = t8675 * t4861;
    let t17626 = t2253 * t4874;
    let t17627 = t2253 * t4885;
    let t17630 = t920 * t1073;
    let t17685 = t1526 * t9483 * t4906;
    (t17554, t17556, t17567, t17573, t17626, t17627, t17630, t17685)
}
