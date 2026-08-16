//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1163/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1163(t3604: f64, t573: f64, t11866: f64, t11909: f64, t1484: f64, t3589: f64, t2058: f64, t933: f64, t2055: f64, t5013: f64, t5021: f64, t5007: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13653 = t573 * t3604;
    let t13655 = t11866 * t13653 * t11909;
    let t13657 = t1484 * t3589;
    let t13659 = t11866 * t13657 * t11909;
    let t13661 = t933 * t2058;
    let t13663 = t933 * t2055;
    let t13665 = t5021 * t5013;
    let t13667 = t5021 * t5007;
    (t13655, t13659, t13661, t13663, t13665, t13667)
}
