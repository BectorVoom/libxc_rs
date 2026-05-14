//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 756/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk756<F: Float>(t2942: F, t7606: F, t2883: F, t735: F, t1066: F, t154: F, t5688: F, t276: F, t2048: F, t2739: F, t2932: F, t5974: F, t2104: F, t2029: F, t2916: F, t178: F, t5723: F) -> (F, F, F, F, F, F, F, F) {
    let t7607 = t2942 * t7606;
    let t7617 = t735 * t2883 / 54.0;
    let t7620 = t154 * t5688 * t1066;
    let t7621 = t276 * t7620;
    let t7628 = t154 * t2048 * t2739;
    let t7630 = t276 * t7628 / 144.0;
    let t7637 = t5974 * t2932;
    let t7639 = 0.57165357490759649296e-3 * t2104 * t7637;
    let t7653 = t2916 * t2029;
    let t7663 = t5723 * t178;
    (t7607, t7617, t7620, t7621, t7630, t7639, t7653, t7663)
}
