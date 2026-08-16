//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1037/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1037(t2171: f64, t5234: f64, t5238: f64, t4738: f64, t5409: f64, t1318: f64, t3854: f64, t6404: f64, t12976: f64, t519: f64, t6418: f64, t13440: f64, t6422: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18409 = t2171 * t5234;
    let t18413 = t2171 * t5238;
    let t18415 = t4738 * t5409;
    let t18435 = t1318 * t3854 * t6404;
    let t18438 = t519 * t12976 * t6418;
    let t18444 = t519 * t13440 * t6422;
    (t18409, t18413, t18415, t18435, t18438, t18444)
}
