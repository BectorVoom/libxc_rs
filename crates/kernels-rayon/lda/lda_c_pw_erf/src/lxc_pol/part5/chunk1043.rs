//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1043/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1043(t1234: f64, t2443: f64, t1386: f64, t6597: f64, t1590: f64, t2379: f64, t1124: f64, t2363: f64, t483: f64, t485: f64, t163: f64, t169: f64, t299: f64, t7287: f64) -> (f64, f64, f64, f64, f64) {
    let t18710 = t2443 * t1234;
    let t18712 = t6597 * t1386;
    let t18735 = t2379 * t1590;
    let t18755 = t1124 * t2363 * t483 * t485;
    let t18761 = t169 * t299 * t7287 * t163;
    (t18710, t18712, t18735, t18755, t18761)
}
