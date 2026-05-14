//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 847/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk847<F: Float>(t10053: F, t25918: F, t1756: F, t352: F, t118: F, t128: F, t1986: F, t1994: F, t6258: F, t2289: F, t38355: F, t8571: F, t8592: F, t34847: F, t9845: F, t38638: F) -> (F, F, F, F, F, F, F) {
    let t46003 = t25918 * t10053;
    let t46005 = t1756 * t352;
    let t46018 = t1994 * t1986 * t118 * t128 * t6258;
    let t46020 = t38355 * t2289;
    let t46022 = t8571 * t8592;
    let t46024 = t34847 * t9845;
    let t46026 = t38638 * t2289;
    (t46003, t46005, t46018, t46020, t46022, t46024, t46026)
}
