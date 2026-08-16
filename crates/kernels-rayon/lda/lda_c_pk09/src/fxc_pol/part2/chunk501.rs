//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 501/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk501(t2758: f64, t462: f64, t451: f64, t2075: f64, t2077: f64, t2079: f64, t2081: f64, t2733: f64, t2736: f64, t2803: f64, t2807: f64, t458: f64) -> (f64, f64, f64, f64) {
    let t2835 = t462 * t2758;
    let t2838 = t451 * t2758;
    let t2845 = t2075 - 0.7661514025603425_f64 * t2803 + t2077 + 0.7661514025603425_f64 * t2807 + t2079 - 0.15282509383508946_f64 * t2733 + t2081 + 0.15282509383508946_f64 * t2736;
    let t2846 = t458 * t2845;
    (t2835, t2838, t2845, t2846)
}
