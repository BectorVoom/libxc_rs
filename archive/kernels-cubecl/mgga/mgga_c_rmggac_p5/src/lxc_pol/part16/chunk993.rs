//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 993/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk993<F: Float>(t2060: F, t6463: F, t305: F, t27101: F, t46533: F, t25854: F, t46537: F, t1756: F, t7778: F, t45418: F, t5271: F, t46258: F, t5162: F) -> (F, F, F, F, F, F, F, F) {
    let t46736 = t2060 * t6463;
    let t46737 = t305 * t46736;
    let t46748 = t27101 * t46533;
    let t46750 = t25854 * t46537;
    let t46764 = t7778 * t1756;
    let t46765 = t305 * t46764;
    let t46770 = t5271 * t45418;
    let t46772 = t5162 * t46258;
    (t46736, t46737, t46748, t46750, t46764, t46765, t46770, t46772)
}
