//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 712/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk712(t6713: f64, t451: f64, t6586: f64, t309: f64, t454: f64, t4993: f64, t462: f64, t1240: f64, t1671: f64, t2056: f64, t471: f64, t2045: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6997 = 0.020557162358903314_f64 * t6713;
    let t7004 = t451 * t6586;
    let t7006 = t309 * t454 * t4993;
    let t7008 = t7004 * t7006 / 3.0_f64;
    let t7013 = t462 * t6586;
    let t7015 = t7013 * t7006 / 3.0_f64;
    let t7017 = t309 * t1671 * t1240;
    let t7019 = t2056 * t7017 / 9.0_f64;
    let t7024 = t471 * t6586;
    let t7026 = t7024 * t7006 / 3.0_f64;
    let t7028 = t2045 * t7017 / 9.0_f64;
    (t6997, t7008, t7015, t7017, t7019, t7026, t7028)
}
