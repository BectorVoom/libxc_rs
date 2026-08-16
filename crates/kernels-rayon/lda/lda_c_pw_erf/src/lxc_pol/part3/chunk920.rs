//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 920/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk920(t325: f64, t3501: f64, t1272: f64, t933: f64, t331: f64, t3487: f64, t1294: f64, t1524: f64, t1446: f64, t3784: f64, t1382: f64, t1518: f64, t211: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9868 = t325 * t3501;
    let t9891 = t933 * t1272;
    let t9893 = t331 * t3487;
    let t9905 = t1524 * t1294;
    let t9909 = t1446 * t3784;
    let t9923 = t211 * t1518 * t1382;
    (t9868, t9891, t9893, t9905, t9909, t9923)
}
