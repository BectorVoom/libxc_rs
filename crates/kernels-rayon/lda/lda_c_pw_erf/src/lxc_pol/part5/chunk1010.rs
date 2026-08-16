//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1010/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1010(t4581: f64, t4763: f64, t571: f64, t6271: f64, t9678: f64, t1318: f64, t3854: f64, t6276: f64, t3802: f64, t519: f64, t6281: f64, t5021: f64, t6815: f64) -> (f64, f64, f64, f64, f64) {
    let t16239 = t4763 * t4581;
    let t16245 = t571 * t9678 * t6271;
    let t16253 = t1318 * t3854 * t6276;
    let t16261 = t519 * t3802 * t6281;
    let t16285 = t5021 * t6815;
    (t16239, t16245, t16253, t16261, t16285)
}
