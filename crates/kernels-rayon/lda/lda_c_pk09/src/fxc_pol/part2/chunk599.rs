//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 599/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk599(t3551: f64, t9: f64, t235: f64, t257: f64, t72: f64, t8: f64, t240: f64, t258: f64, t4787: f64, t4789: f64, t252: f64, t272: f64) -> (f64, f64, f64) {
    let t4792 = t9 * t3551;
    let t4793 = t235 * t4792;
    let t4796 = 1.0_f64 / t72 / t257;
    let t4797 = t8 * t4796;
    let t4800 = 2.6672246125319017_f64 * t4787 + 13.705920266221307_f64 * t4789 - 11.375235355360967_f64 * t258 + 0.5507137659888112_f64 * t4793 + 0.00024419928681528166_f64 * t240 * t4797;
    let t4801 = t4800 * t252;
    let t4803 = 1.28_f64 * t4801 * t272;
    (t4793, t4801, t4803)
}
