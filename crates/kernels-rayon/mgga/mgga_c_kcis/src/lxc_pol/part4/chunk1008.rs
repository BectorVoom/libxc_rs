//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1008/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1008(t1484: f64, t3251: f64, t1098: f64, t3811: f64, t11402: f64, t513: f64, t1416: f64, t3820: f64, t1317: f64, t3838: f64, t11407: f64, t3843: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11723 = t3251 * t1484;
    let t11725 = t1098 * t3811;
    let t11727 = t11402 * t513;
    let t11730 = t3820 * t1416;
    let t11736 = t1317 * t3838;
    let t11746 = 0.12841111111111111111e-1_f64 * t11407;
    let t11767 = t1098 * t3843;
    (t11723, t11725, t11727, t11730, t11736, t11746, t11767)
}
