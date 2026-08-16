//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3908/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3908<F: Float>(t1398: F, t73820: F, t2782: F, t47371: F, t6862: F, t10022: F, t22315: F, t46457: F, t136: F, t2457: F, t47429: F, t14193: F, t22016: F, t46510: F, t46515: F, t46518: F, t48076: F, t48079: F, t48081: F, t48085: F, t48089: F, t5658: F, t5735: F) -> F {
    let t75047 = t73820 * t1398;
    let t75049 = t2782 * t47371 * t75047;
    let t75051 = t6862 * t1398;
    let t75053 = t2782 * t10022 * t75051;
    let t75060 = t46457 * t22315;
    let t75068 = t47429 * t6862 * t136 * t2457;
    let t75070 = -F::cast_from(0.11565819519348392139e-2_f64) * t46510 + F::cast_from(0.65854491829355115984e-1_f64) * t75049 - F::cast_from(0.65854491829355115984e-1_f64) * t75053 - t46515 - F::cast_from(0.65854491829355115984e-1_f64) * t48076 + F::cast_from(0.52039682876708176102e-1_f64) * t48079 + F::cast_from(0.2601984143835408805e-2_f64) * t48081 + F::cast_from(0.39029762157531132076e-1_f64) * t48085 + F::cast_from(0.46263278077393568556e-2_f64) * t48089 + F::cast_from(0.39029762157531132074e-1_f64) * t75060 - F::cast_from(0.15805078039045227836e2_f64) * t14193 * t5735 * t22016 * t5658 + t46518 + F::cast_from(0.23131639038696784277e-2_f64) * t75068;
    t75070
}
