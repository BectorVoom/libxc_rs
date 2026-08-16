//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 392/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk392(t494: f64, t529: f64, t542: f64, t1440: f64, t1325: f64, t510: f64, t518: f64) -> (f64, f64, f64, f64) {
    let t1442 = t529 * t494 * t542;
    let t1443 = t1440 * t1442;
    let t1445 = 8.0_f64 / 15.0_f64 * t1325 * t1443;
    let t1446 = t510 * t518;
    (t1442, t1443, t1445, t1446)
}
