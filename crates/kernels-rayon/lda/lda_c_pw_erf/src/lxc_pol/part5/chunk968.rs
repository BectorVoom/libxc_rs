//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 968/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk968(t13517: f64, t1184: f64, t2177: f64, t519: f64, t521: f64, t4729: f64, t511: f64, t2061: f64, t830: f64, t11845: f64, t2062: f64, t1351: f64, t588: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13518 = 8.0_f64 / 135.0_f64 * t13517;
    let t13523 = t519 * t1184 * t521 * t2177;
    let t13550 = t511 * t4729;
    let t13551 = 4.0_f64 / 45.0_f64 * t13550;
    let t13562 = t2061 * t830;
    let t13564 = t11845 * t2062;
    let t13631 = t588 * t1351;
    (t13518, t13523, t13551, t13562, t13564, t13631)
}
