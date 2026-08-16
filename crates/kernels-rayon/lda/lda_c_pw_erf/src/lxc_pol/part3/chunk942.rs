//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 942/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk942(t10605: f64, t1487: f64, t571: f64, t3715: f64, t4062: f64, t1472: f64, t4059: f64, t4063: f64, t1325: f64, t3731: f64, t3787: f64, t1340: f64, t3783: f64, t519: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10607 = t571 * t10605 * t1487;
    let t10610 = t571 * t4062 * t3715;
    let t10612 = t1472 * t4059;
    let t10614 = t1472 * t4063;
    let t10617 = t1325 * t3787 * t3731;
    let t10620 = t519 * t3783 * t1340;
    (t10607, t10610, t10612, t10614, t10617, t10620)
}
