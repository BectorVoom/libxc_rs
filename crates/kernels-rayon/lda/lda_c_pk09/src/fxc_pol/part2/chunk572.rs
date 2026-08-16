//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 572/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk572(t3998: f64, t891: f64, t3997: f64, t3161: f64, t61: f64, t96: f64, t3523: f64, t55: f64, t130: f64, t3104: f64, t893: f64, t132: f64, t3677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3999 = t891 * t3998;
    let t4000 = t3997 * t3999;
    let t4001 = 5.40024514194619_f64 * t4000;
    let t4002 = t61 * t3161;
    let t4003 = t96 * t4002;
    let t4004 = t3523 * t4003;
    let t4005 = 44.15969676259812_f64 * t4004;
    let t4006 = t55 * t55;
    let t4007 = 1.0_f64 / t4006;
    let t4008 = t130 * t4007;
    let t4021 = t3104 * t893;
    let t4023 = t3677 * t132;
    (t4000, t4001, t4004, t4005, t4007, t4008, t4021, t4023)
}
