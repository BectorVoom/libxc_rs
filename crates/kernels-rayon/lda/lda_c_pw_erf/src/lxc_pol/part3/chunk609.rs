//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 609/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk609(t1268: f64, t3482: f64, t1245: f64, t2954: f64, t538: f64, t3412: f64, t1253: f64, t325: f64, t1243: f64, t3477: f64, t11: f64, t503: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3483 = t1268 * t3482;
    let t3486 = t1245 * t2954;
    let t3487 = t538 * t3486;
    let t3490 = t538 * t3412;
    let t3493 = t325 * t1253;
    let t3495 = t1243 * t3477;
    let t3496 = t11 * t3495;
    let t3498 = t1243 * t3482;
    let t3499 = t11 * t3498;
    let t3501 = t503 * t3486;
    (t3483, t3487, t3490, t3493, t3495, t3496, t3498, t3499, t3501)
}
