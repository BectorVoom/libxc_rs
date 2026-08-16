//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 831/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk831(t1217: f64, t858: f64, t34: f64, t92: f64, t93: f64, t108: f64, t2268: f64, t2271: f64, t2274: f64, t2277: f64, t39: f64, t4356: f64, t4371: f64, t462: f64, t659: f64, t661: f64, t753: f64, t754: f64, t940: f64, t945: f64, t951: f64, t954: f64) -> (f64, f64, f64, f64) {
    let t5806 = t858 * t1217;
    let t5812 = t92 * t34;
    let t5823 = t93 * t34;
    let t5833 = (40.0_f64 / 27.0_f64 * t753 * t940 + 80.0_f64 / 9.0_f64 * t5812 * t4356 + 20.0_f64 / 9.0_f64 * t2268 * t945 + 8.0_f64 / 3.0_f64 * t659 * t462 - 8.0_f64 * t2271 * t39 + 40.0_f64 / 27.0_f64 * t754 * t951 - 80.0_f64 / 9.0_f64 * t5823 * t4371 + 20.0_f64 / 9.0_f64 * t2274 * t954 - 8.0_f64 / 3.0_f64 * t661 * t462 + 8.0_f64 * t2277 * t39) * t108;
    (t5806, t5812, t5823, t5833)
}
