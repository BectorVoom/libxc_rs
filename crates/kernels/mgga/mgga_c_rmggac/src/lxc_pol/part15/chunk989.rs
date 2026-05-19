//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 989/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk989<F: Float>(t46176: F, t797: F, t265: F, t9908: F, t46128: F, t851: F, t854: F, t3810: F, t46184: F, t3839: F, t46180: F, t36188: F, t36190: F, t36201: F, t36205: F, t41371: F, t41373: F, t41378: F, t41380: F, t41381: F, t43623: F) -> F {
    let t46300 = t797 * t46176;
    let t46302 = t9908 * t265;
    let t46305 = t851 * t46128;
    let t46307 = t854 * t46176;
    let t46309 = t3810 * t46184;
    let t46311 = t3839 * t46180;
    let t46313 = t43623 - F::cast_from(0.32452821145703643272e-2_f64) * t36188 + F::cast_from(0.38943385374844371927e-2_f64) * t36190 + t36201 + F::cast_from(0.53218852008283593619e-1_f64) * t41371 + F::cast_from(0.53218852008283593619e-1_f64) * t41373 - t36205 - F::cast_from(0.39914139006212695213e-1_f64) * t46300 + F::cast_from(0.26609426004141796809e-1_f64) * t46302 - t41378 + t41380 + F::cast_from(0.56448240417072397695e-3_f64) * t41381 + F::cast_from(0.88507694033737208925e-3_f64) * t46305 - F::cast_from(0.10620923284048465071e-2_f64) * t46307 + F::cast_from(0.74346462988339255496e-2_f64) * t46309 + F::cast_from(0.35403077613494883571e-2_f64) * t46311;
    t46313
}
