//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 806/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk806(t100: f64, t1664: f64, t142: f64, t1832: f64, t1568: f64, t1809: f64, t1849: f64, t925: f64, t1814: f64, t474: f64, t763: f64, t426: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5490 = t1664 * t100;
    let t5495 = t142 * t1832;
    let t5499 = t1809 * t1568;
    let t5502 = t1849 * t925;
    let t5504 = t1814 * t925;
    let t5505 = 0.6495611111111111_f64 * t5504;
    let t5506 = t474 * t763;
    let t5507 = t426 * t5506;
    (t5490, t5495, t5499, t5502, t5505, t5506, t5507)
}
