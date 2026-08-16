//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1160/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1160(t325: f64, t4681: f64, t4667: f64, t4606: f64, t4677: f64, t11: f64, t12264: f64, t1349: f64, t12153: f64, t1953: f64, t2967: f64, t743: f64, t9410: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13585 = t325 * t4681;
    let t13587 = t325 * t4667;
    let t13589 = t4606 * t4677;
    let t13592 = t11 * t1349 * t12264;
    let t13595 = t1953 * t1349 * t12153;
    let t13598 = t9410 * t743 * t2967;
    (t13585, t13587, t13589, t13592, t13595, t13598)
}
