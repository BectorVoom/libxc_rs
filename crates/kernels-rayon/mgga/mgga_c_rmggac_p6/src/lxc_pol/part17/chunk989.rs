//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 989/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk989(t46176: f64, t797: f64, t265: f64, t9908: f64, t46128: f64, t851: f64, t854: f64, t3810: f64, t46184: f64, t3839: f64, t46180: f64, t36188: f64, t36190: f64, t36201: f64, t36205: f64, t41371: f64, t41373: f64, t41378: f64, t41380: f64, t41381: f64, t43623: f64) -> f64 {
    let t46300 = t797 * t46176;
    let t46302 = t9908 * t265;
    let t46305 = t851 * t46128;
    let t46307 = t854 * t46176;
    let t46309 = t3810 * t46184;
    let t46311 = t3839 * t46180;
    let t46313 = t43623 - 0.32452821145703643272e-2_f64 * t36188 + 0.38943385374844371927e-2_f64 * t36190 + t36201 + 0.53218852008283593619e-1_f64 * t41371 + 0.53218852008283593619e-1_f64 * t41373 - t36205 - 0.39914139006212695213e-1_f64 * t46300 + 0.26609426004141796809e-1_f64 * t46302 - t41378 + t41380 + 0.56448240417072397695e-3_f64 * t41381 + 0.88507694033737208925e-3_f64 * t46305 - 0.10620923284048465071e-2_f64 * t46307 + 0.74346462988339255496e-2_f64 * t46309 + 0.35403077613494883571e-2_f64 * t46311;
    t46313
}
