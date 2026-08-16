//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 202/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk202(t188: f64, t655: f64, t659: f64, t694: f64, t183: f64, t186: f64, t89: f64, t132: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t697 = t655 * t188 - t659 * t694 / 2.0_f64;
    let t698 = t183 * t183;
    let t699 = 1.0_f64 / t186;
    let t701 = -t698 * t699 + 1.0_f64;
    let t702 = 1.0_f64 / t701;
    let t703 = t697 * t702;
    let t704 = t703 * t89;
    let t707 = t132 * t61;
    (t698, t699, t701, t702, t703, t704, t707)
}
