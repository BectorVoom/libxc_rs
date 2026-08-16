//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1000/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1000(t11677: f64, t1318: f64, t3899: f64, t5366: f64, t1466: f64, t2191: f64, t3655: f64, t9217: f64, t1966: f64, t2961: f64, t4619: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11678 = 8.0_f64 / 45.0_f64 * t11677;
    let t11680 = t1318 * t3899 * t5366;
    let t11681 = 8.0_f64 / 15.0_f64 * t11680;
    let t11685 = 4.0_f64 / 15.0_f64 * t1318 * t1466 * t2191 * t3655;
    let t11686 = 4.0_f64 / 45.0_f64 * t9217;
    let t11687 = t1966 * t2961;
    let t11691 = t4619 * t945;
    (t11678, t11681, t11685, t11686, t11687, t11691)
}
