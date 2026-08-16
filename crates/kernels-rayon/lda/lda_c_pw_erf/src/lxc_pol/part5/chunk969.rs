//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 969/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk969(t1370: f64, t3604: f64, t3586: f64, t3589: f64, t1351: f64, t213: f64, t573: f64, t1484: f64, t2058: f64, t933: f64, t2055: f64, t1950: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13635 = t1370 * t3604;
    let t13639 = t3586 * t3589;
    let t13643 = t213 * t1351;
    let t13653 = t573 * t3604;
    let t13657 = t1484 * t3589;
    let t13661 = t933 * t2058;
    let t13663 = t933 * t2055;
    let t13710 = t925 * t1950;
    (t13635, t13639, t13643, t13653, t13657, t13661, t13663, t13710)
}
