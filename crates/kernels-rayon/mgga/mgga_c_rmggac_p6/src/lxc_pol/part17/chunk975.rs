//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 975/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk975(t6441: f64, t649: f64, t7599: f64, t6421: f64, t8746: f64, t41130: f64, t6425: f64, t36088: f64, t36090: f64, t41195: f64, t41231: f64, t41234: f64, t41242: f64, t43528: f64, t43558: f64, t46123: f64, t46126: f64, t46130: f64, t46133: f64, t46135: f64) -> (f64, f64, f64) {
    let t46139 = t649 * t6441;
    let t46140 = t7599 * t46139;
    let t46142 = t649 * t6421;
    let t46143 = t8746 * t46142;
    let t46146 = t41130 * t649 * t6425;
    let t46148 = -0.10584045078201074568e-3_f64 * t46123 + 0.68186654135613354324e-2_f64 * t46126 - 0.90915538847484472432e-2_f64 * t46130 - t43528 - 0.79828278012425390428e-1_f64 * t41195 + 0.34093327067806677162e-2_f64 * t46133 - 0.45457769423742236216e-2_f64 * t46135 + 0.88704377798256624948e-3_f64 * t36088 - 0.10348844076463272911e-2_f64 * t36090 + t41231 - t41234 + t41242 + t43558 - 0.13637330827122670865e-1_f64 * t46140 + 0.22728884711871118108e-1_f64 * t46143 + 0.1814407727691612783e-2_f64 * t46146;
    (t46139, t46142, t46148)
}
