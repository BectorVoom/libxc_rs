//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 408/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk408(t1519: f64, t185: f64, t549: f64, t209: f64, t184: f64) -> (f64, f64, f64, f64) {
    let t1521 = 4.0_f64 / 135.0_f64 * t185 * t1519;
    let t1522 = t549 * t549;
    let t1523 = t1522 * t209;
    let t1524 = t1523 * t184;
    (t1521, t1522, t1523, t1524)
}
