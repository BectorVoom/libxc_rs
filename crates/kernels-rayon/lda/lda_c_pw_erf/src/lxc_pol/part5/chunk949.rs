//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 949/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk949(t1953: f64, t790: f64, t10967: f64, t21: f64, t2095: f64, t1977: f64, t8930: f64, t191: f64, t24: f64, t1267: f64, t3476: f64, t3515: f64, t3518: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11834 = t1953 * t790;
    let t11845 = t21 * t10967;
    let t11846 = t11845 * t2095;
    let t11848 = t8930 * t1977;
    let t11854 = t21 * t24 * t191;
    let t11855 = t1267 * t3476;
    let t11861 = t3515 * t3518;
    (t11834, t11845, t11846, t11848, t11854, t11855, t11861)
}
