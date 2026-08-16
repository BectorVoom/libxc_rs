//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1178/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1178<F: Float>(t12985: F, t9577: F, t41189: F, t4134: F, t1489: F, t41083: F, t133: F, t1484: F, t41214: F, t6600: F, t1512: F, t41362: F) -> (F, F, F, F, F) {
    let t46764 = t9577 * t12985;
    let t46772 = t41189 * t4134;
    let t46790 = t41083 * t1489;
    let t46806 = t41214 * t133 * t6600 * t1484;
    let t46876 = t41362 * t1512;
    (t46764, t46772, t46790, t46806, t46876)
}
