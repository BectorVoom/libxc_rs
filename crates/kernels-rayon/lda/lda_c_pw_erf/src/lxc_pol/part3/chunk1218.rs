//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1218/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1218(t4713: f64, t473: f64, t483: f64, t485: f64, t1131: f64, t5474: f64, t1910: f64, t2910: f64, t1124: f64, t1904: f64, t5470: f64, t1191: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14382 = t473 * t4713 * t483 * t485;
    let t14385 = t5474 * t1131 * t485;
    let t14386 = 0.01185233419734569_f64 * t14385;
    let t14388 = t1910 * t2910 * t485;
    let t14392 = t1124 * t1904 * t483 * t485;
    let t14393 = 0.01975389032890948_f64 * t14392;
    let t14395 = t5470 * t1131 * t485;
    let t14397 = t1191 * t780;
    (t14382, t14386, t14388, t14393, t14395, t14397)
}
