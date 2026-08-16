//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 385/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk385(t1402: f64, t1403: f64, t186: f64, t211: f64, t653: f64, t656: f64, t156: f64, t254: f64) -> (f64, f64, f64, f64, f64) {
    let t1404 = t1402 * t1403;
    let t1405 = t186 * t1404;
    let t1407 = 4.0_f64 / 15.0_f64 * t211 * t1405;
    let t1409 = 4.0_f64 / 9.0_f64 * t653 * t656;
    let t1410 = t254 * t156;
    (t1404, t1405, t1407, t1409, t1410)
}
