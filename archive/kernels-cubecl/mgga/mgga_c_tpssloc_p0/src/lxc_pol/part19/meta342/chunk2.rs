//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1221/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1221<F: Float>(t2582: F, t9541: F, t786: F, t9580: F, t2578: F, t9546: F, t9555: F, t2573: F, t41008: F, t2566: F, t2570: F, t9551: F) -> (F, F, F, F, F) {
    let t41187 = t9541 * t2582;
    let t41189 = t9580 * t786;
    let t41190 = t41189 * t2578;
    let t41192 = t9546 * t9555;
    let t41194 = t41008 * t2573;
    let t41196 = t2566 * t2570;
    let t41197 = t41196 * t9551;
    (t41187, t41190, t41192, t41194, t41197)
}
