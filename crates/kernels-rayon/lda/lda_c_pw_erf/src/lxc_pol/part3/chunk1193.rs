//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1193/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1193(t10030: f64, t4476: f64, t2070: f64, t807: f64, t185: f64, t3679: f64, t795: f64, t834: f64, t211: f64, t548: f64, t812: f64, t10632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14041 = t10030 * t4476;
    let t14042 = 32.0_f64 / 45.0_f64 * t14041;
    let t14043 = t2070 * t807;
    let t14044 = t185 * t14043;
    let t14045 = 16.0_f64 / 405.0_f64 * t14044;
    let t14047 = 4.0_f64 / 5.0_f64 * t795 * t3679;
    let t14048 = t2070 * t834;
    let t14049 = t211 * t14048;
    let t14050 = 16.0_f64 / 405.0_f64 * t14049;
    let t14052 = t548 * t2070 * t812;
    let t14053 = 32.0_f64 / 405.0_f64 * t14052;
    let t14054 = 8.0_f64 / 27.0_f64 * t10632;
    (t14042, t14045, t14047, t14050, t14053, t14054)
}
