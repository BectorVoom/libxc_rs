//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 493/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk493<F: Float>(t210: F, t214: F, t2379: F, t786: F, t792: F, t118: F, t776: F, t794: F, t2553: F, t59: F, t835: F, t154: F) -> (F, F, F, F, F, F, F) {
    let t2573 = t210 * t214 * t2379;
    let t2576 = t792 * t786;
    let t2578 = t118 * t794 * t776;
    let t2579 = t2576 * t2578;
    let t2582 = t210 * t214 * t2553;
    let t2585 = t59 * t835;
    let t2586 = t2585 * t154;
    (t2573, t2576, t2578, t2579, t2582, t2585, t2586)
}
