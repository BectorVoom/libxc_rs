//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 636/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk636<F: Float>(t1163: F, t4959: F, t1165: F, t3196: F, t540: F, t4210: F, t530: F, t3194: F, t1181: F, t3169: F, t535: F, t3176: F, t4643: F) -> (F, F, F, F, F, F, F, F) {
    let t4961 = F::cast_from(0.85748036236139473944e-3_f64) * t1163 * t4959;
    let t4963 = t1165 * t540 * t3196;
    let t4967 = t1165 * t530 * t4210;
    let t4969 = F::cast_from(0.17149607247227894789e-2_f64) * t3194 * t4967;
    let t4971 = t1165 * t530 * t3196;
    let t4975 = t1181 * t530 * t3169;
    let t4978 = t535 * t3196;
    let t4979 = t1181 * t4978;
    let t4982 = t4643 * t3176;
    (t4961, t4963, t4967, t4969, t4971, t4975, t4979, t4982)
}
