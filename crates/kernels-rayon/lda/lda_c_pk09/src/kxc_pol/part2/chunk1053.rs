//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1053/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1053(t11461: f64, t476: f64, t11248: f64, t2747: f64, t747: f64, t481: f64, t2743: f64, t1995: f64, t2938: f64, t7340: f64, t452: f64, t1775: f64, t309: f64, t454: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11462 = t476 * t11461;
    let t11464 = t476 * t11248;
    let t11466 = t747 * t2747;
    let t11467 = t481 * t11466;
    let t11469 = t747 * t2743;
    let t11470 = t1995 * t11469;
    let t11472 = t2938 * t7340;
    let t11473 = t11472 * t452;
    let t11475 = t309 * t454 * t1775;
    (t11462, t11464, t11466, t11467, t11469, t11470, t11473, t11475)
}
