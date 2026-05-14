//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 853/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk853<F: Float>(t1737: F, t2064: F, t3814: F, t1763: F, t36292: F, t305: F, t2067: F, t30526: F, t9885: F, t2061: F, t9908: F, t15093: F, t9005: F, t1704: F, t325: F, t2057: F) -> (F, F, F, F, F, F, F, F, F) {
    let t46055 = t2064 * t1737;
    let t46056 = t3814 * t46055;
    let t46058 = t36292 * t1763;
    let t46059 = t305 * t46058;
    let t46062 = t30526 * t2067 * t9885;
    let t46064 = t9908 * t2061;
    let t46066 = t15093 * t9005;
    let t46068 = t1704 * t325;
    let t46069 = t46068 * t2057;
    (t46055, t46056, t46058, t46059, t46062, t46064, t46066, t46068, t46069)
}
