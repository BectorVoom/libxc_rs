//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 658/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk658(t1124: f64, t780: f64, t483: f64, t485: f64, t1904: f64, t473: f64, t1131: f64, t1910: f64, t142: f64, t1832: f64, t1849: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5470 = t1124 * t780;
    let t5472 = t5470 * t483 * t485;
    let t5474 = t473 * t1904;
    let t5477 = 0.003950778065781896_f64 * t5474 * t483 * t485;
    let t5479 = t1910 * t1131 * t485;
    let t5495 = t142 * t1832;
    let t5502 = t1849 * t925;
    (t5470, t5472, t5474, t5477, t5479, t5495, t5502)
}
