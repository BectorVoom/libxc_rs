//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 561/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk561<F: Float>(t115: F, t2770: F, t282: F, t2769: F, t123: F, t2752: F, t323: F, t1659: F, t2670: F, t297: F, t2606: F, t287: F) -> (F, F, F, F, F, F, F) {
    let t2772 = t282 * t2770 * t115;
    let t2773 = t2769 * t2772;
    let t2774 = t2752 * t123;
    let t2775 = t323 * t2774;
    let t2778 = t1659 * t2772;
    let t2780 = t2670 * t123 * t297;
    let t2781 = t323 * t2780;
    let t2785 = t287 * t2606 * t297;
    (t2773, t2774, t2775, t2778, t2780, t2781, t2785)
}
