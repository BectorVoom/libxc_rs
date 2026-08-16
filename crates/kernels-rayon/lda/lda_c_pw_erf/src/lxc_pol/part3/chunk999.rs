//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 999/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk999(t163: f64, t169: f64, t2198: f64, t717: f64, t299: f64, t5433: f64, t1440: f64, t2186: f64, t3545: f64, t519: f64, t1318: f64, t2192: f64, t9432: f64) -> (f64, f64, f64, f64) {
    let t11666 = t169 * t717 * t2198 * t163;
    let t11667 = 0.07184540406152766_f64 * t11666;
    let t11670 = t169 * t299 * t5433 * t163;
    let t11675 = 4.0_f64 / 15.0_f64 * t519 * t1440 * t2186 * t3545;
    let t11677 = t1318 * t9432 * t2192;
    (t11667, t11670, t11675, t11677)
}
