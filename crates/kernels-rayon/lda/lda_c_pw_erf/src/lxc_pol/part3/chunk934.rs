//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 934/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk934(t1449: f64, t3738: f64, t519: f64, t1335: f64, t3762: f64, t571: f64, t1318: f64, t3420: f64, t3854: f64, t4049: f64, t581: f64, t549: f64, t593: f64) -> (f64, f64, f64, f64, f64) {
    let t10350 = t519 * t1449 * t3738;
    let t10361 = t571 * t3762 * t1335;
    let t10371 = t1318 * t3854 * t3420;
    let t10379 = t4049 * t581;
    let t10392 = t549 * t593;
    (t10350, t10361, t10371, t10379, t10392)
}
