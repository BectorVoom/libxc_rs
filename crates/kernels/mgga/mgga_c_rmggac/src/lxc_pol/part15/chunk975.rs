//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 975/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk975<F: Float>(t6441: F, t649: F, t7599: F, t6421: F, t8746: F, t41130: F, t6425: F, t36088: F, t36090: F, t41195: F, t41231: F, t41234: F, t41242: F, t43528: F, t43558: F, t46123: F, t46126: F, t46130: F, t46133: F, t46135: F) -> (F, F, F) {
    let t46139 = t649 * t6441;
    let t46140 = t7599 * t46139;
    let t46142 = t649 * t6421;
    let t46143 = t8746 * t46142;
    let t46146 = t41130 * t649 * t6425;
    let t46148 = -F::cast_from(0.10584045078201074568e-3_f64) * t46123 + F::cast_from(0.68186654135613354324e-2_f64) * t46126 - F::cast_from(0.90915538847484472432e-2_f64) * t46130 - t43528 - F::cast_from(0.79828278012425390428e-1_f64) * t41195 + F::cast_from(0.34093327067806677162e-2_f64) * t46133 - F::cast_from(0.45457769423742236216e-2_f64) * t46135 + F::cast_from(0.88704377798256624948e-3_f64) * t36088 - F::cast_from(0.10348844076463272911e-2_f64) * t36090 + t41231 - t41234 + t41242 + t43558 - F::cast_from(0.13637330827122670865e-1_f64) * t46140 + F::cast_from(0.22728884711871118108e-1_f64) * t46143 + F::cast_from(0.1814407727691612783e-2_f64) * t46146;
    (t46139, t46142, t46148)
}
