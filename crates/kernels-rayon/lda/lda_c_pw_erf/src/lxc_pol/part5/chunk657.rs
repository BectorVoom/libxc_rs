//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 657/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk657(t164: f64, t5446: f64, t1901: f64, t479: f64, t1905: f64, t163: f64, t169: f64, t2198: f64, t299: f64, t717: f64, t780: f64, t1138: f64, t1597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5448 = 0.06301081444628223_f64 * t5446 * t164;
    let t5449 = t1901 * t479;
    let t5455 = 0.06301081444628223_f64 * t1905 * t479;
    let t5459 = 0.017961351015381915_f64 * t169 * t299 * t2198 * t163;
    let t5466 = t717 * t780;
    let t5468 = t5466 * t1138 * t1597;
    (t5448, t5449, t5455, t5459, t5466, t5468)
}
