//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 840/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk840<F: Float>(t11669: F, t2660: F, t2696: F, t277: F, t2977: F, t775: F, t761: F, t778: F, t13: F, t2: F, t3151: F, t3157: F, t721: F) -> (F, F, F, F, F) {
    let t11778 = F::cast_from(0.19263893255070628431e1_f64) * t11669 * t2696 * t2660;
    let t11780 = F::cast_from(480.0_f64) * t2977 * t277;
    let t11784 = t775 * t775;
    let t11787 = t761 * t761;
    let t11788 = t778 * t778;
    let t11792 = F::cast_from(0.24955700379505800916e5_f64) * t13 / t11784 * t11787 / t11788;
    let t11795 = t3157 * t2 * t3151 * t721;
    (t11778, t11780, t11787, t11792, t11795)
}
