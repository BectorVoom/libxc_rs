//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 537/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk537(t870: f64, t873: f64, t220: f64, t872: f64, t206: f64, t887: f64, t217: f64, t20: f64, t2394: f64, t62: f64, t212: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2720 = t870 * t873;
    let t2724 = 1.0_f64 / t872 / t220;
    let t2725 = t206 * t2724;
    let t2726 = t887 * t887;
    let t2727 = t217 * t217;
    let t2728 = 1.0_f64 / t2727;
    let t2729 = t2726 * t2728;
    let t2733 = t62 * t2394 * t20;
    let t2739 = 1.0_f64 / t879 / t212;
    (t2720, t2724, t2725, t2726, t2727, t2728, t2729, t2733, t2739)
}
