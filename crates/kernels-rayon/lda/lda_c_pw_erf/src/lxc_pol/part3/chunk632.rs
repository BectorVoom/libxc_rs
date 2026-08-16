//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 632/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk632(t3727: f64, t577: f64, t1390: f64, t1392: f64, t494: f64, t1440: f64, t1325: f64, t1340: f64, t1449: f64, t519: f64, t1460: f64, t2954: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3729 = 4.0_f64 / 15.0_f64 * t3727 * t577;
    let t3731 = t1390 * t494 * t1392;
    let t3732 = t1440 * t3731;
    let t3734 = 8.0_f64 / 5.0_f64 * t1325 * t3732;
    let t3735 = t1449 * t1340;
    let t3736 = t519 * t3735;
    let t3737 = 16.0_f64 / 45.0_f64 * t3736;
    let t3738 = t1460 * t2954;
    (t3729, t3731, t3732, t3734, t3735, t3736, t3737, t3738)
}
