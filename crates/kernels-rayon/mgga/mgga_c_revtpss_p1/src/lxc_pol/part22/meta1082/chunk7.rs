//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3908/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3908(t1398: f64, t73820: f64, t2782: f64, t47371: f64, t6862: f64, t10022: f64, t22315: f64, t46457: f64, t136: f64, t2457: f64, t47429: f64, t14193: f64, t22016: f64, t46510: f64, t46515: f64, t46518: f64, t48076: f64, t48079: f64, t48081: f64, t48085: f64, t48089: f64, t5658: f64, t5735: f64) -> f64 {
    let t75047 = t73820 * t1398;
    let t75049 = t2782 * t47371 * t75047;
    let t75051 = t6862 * t1398;
    let t75053 = t2782 * t10022 * t75051;
    let t75060 = t46457 * t22315;
    let t75068 = t47429 * t6862 * t136 * t2457;
    let t75070 = -0.11565819519348392139e-2_f64 * t46510 + 0.65854491829355115984e-1_f64 * t75049 - 0.65854491829355115984e-1_f64 * t75053 - t46515 - 0.65854491829355115984e-1_f64 * t48076 + 0.52039682876708176102e-1_f64 * t48079 + 0.2601984143835408805e-2_f64 * t48081 + 0.39029762157531132076e-1_f64 * t48085 + 0.46263278077393568556e-2_f64 * t48089 + 0.39029762157531132074e-1_f64 * t75060 - 0.15805078039045227836e2_f64 * t14193 * t5735 * t22016 * t5658 + t46518 + 0.23131639038696784277e-2_f64 * t75068;
    t75070
}
