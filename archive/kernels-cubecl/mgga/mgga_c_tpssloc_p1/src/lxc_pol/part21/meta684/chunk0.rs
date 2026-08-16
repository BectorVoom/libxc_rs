//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2497/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2497<F: Float>(t133: F, t1484: F, t41214: F, t6600: F, t12998: F, t46766: F, t686: F, t776: F, t12984: F, t2553: F, t12990: F, t13012: F) -> (F, F, F, F) {
    let t46806 = t41214 * t133 * t6600 * t1484;
    let t46819 = t12998 * t686 * t46766 * t776;
    let t46828 = t12998 * t686 * t12984 * t2553;
    let t46830 = t13012 * t12990;
    (t46806, t46819, t46828, t46830)
}
