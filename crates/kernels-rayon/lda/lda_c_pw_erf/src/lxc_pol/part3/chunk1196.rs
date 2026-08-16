//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1196/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1196(t1313: f64, t348: f64, t504: f64, t5127: f64, t519: f64, t11983: f64, t1318: f64, t1403: f64, t549: f64, t833: f64, t4039: f64, t795: f64) -> (f64, f64, f64) {
    let t14083 = 4.0_f64 / 15.0_f64 * t519 * t1313 * t5127 * t504 * t348;
    let t14088 = 24.0_f64 / 5.0_f64 * t1318 * t11983 * t833 * t1403 * t549;
    let t14089 = t795 * t4039;
    (t14083, t14088, t14089)
}
