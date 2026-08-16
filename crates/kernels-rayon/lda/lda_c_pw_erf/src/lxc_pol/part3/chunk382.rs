//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 382/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk382(t1391: f64, t1392: f64, t186: f64, t185: f64, t514: f64, t550: f64) -> (f64, f64, f64, f64) {
    let t1393 = t1391 * t1392;
    let t1394 = t186 * t1393;
    let t1396 = 4.0_f64 / 15.0_f64 * t185 * t1394;
    let t1397 = t514 * t550;
    (t1393, t1394, t1396, t1397)
}
