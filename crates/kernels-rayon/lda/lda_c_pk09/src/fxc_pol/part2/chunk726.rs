//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 726/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk726(t2042: f64, t7432: f64, t6501: f64, t6505: f64, t6522: f64, t6319: f64, t6325: f64, t6547: f64, t6464: f64, t1852: f64, t6287: f64, t1800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7433 = t7432 * t2042;
    let t7437 = 4.0_f64 * t6501;
    let t7438 = 4.0_f64 * t6505;
    let t7442 = 5.333333333333333_f64 * t6522;
    let t7446 = 0.821419393556371_f64 * t6319;
    let t7453 = 0.5476129290375806_f64 * t6325;
    let t7454 = 0.4444444444444444_f64 * t6547;
    let t7459 = 0.18253764301252687_f64 * t6464;
    let t7466 = t1852 * t6287;
    let t7467 = t7466 * t1800;
    (t7433, t7437, t7438, t7442, t7446, t7453, t7454, t7459, t7467)
}
