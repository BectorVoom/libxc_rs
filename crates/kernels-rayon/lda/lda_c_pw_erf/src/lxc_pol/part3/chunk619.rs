//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 619/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk619(t2967: f64, t3604: f64, t1371: f64, t1351: f64, t352: f64, t954: f64) -> (f64, f64, f64, f64) {
    let t3605 = t3604 * t2967;
    let t3606 = t1371 * t3605;
    let t3609 = t1351 * t352;
    let t3610 = t3609 * t954;
    (t3605, t3606, t3609, t3610)
}
