//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 656/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk656(t1625: f64, t5805: f64, t1637: f64, t5755: f64, t1240: f64, t1610: f64, t93: f64, t5123: f64, t328: f64, t5759: f64, t1579: f64, t305: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5806 = t5805 * t1625;
    let t5812 = t1637 * t5755 / 6.0_f64;
    let t5813 = t1610 * t1240;
    let t5814 = t93 * t5813;
    let t5815 = t5123 * t5814;
    let t5817 = t328 * t5759;
    let t5819 = t1579 * t305;
    (t5806, t5812, t5814, t5815, t5817, t5819)
}
