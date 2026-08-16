//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1187/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1187(t17423: f64, t1325: f64, t2437: f64, t34: f64, t4829: f64, t4738: f64, t6433: f64, t17426: f64, t18555: f64, t2480: f64, t6867: f64, t6875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21564 = 16.0_f64 / 45.0_f64 * t17423;
    let t21568 = 8.0_f64 / 15.0_f64 * t1325 * t4829 * t2437 * t34;
    let t21570 = 16.0_f64 / 5.0_f64 * t4738 * t6433;
    let t21571 = 4.0_f64 / 15.0_f64 * t17426;
    let t21573 = 4.0_f64 / 5.0_f64 * t18555 * t2480;
    let t21575 = 4.0_f64 / 5.0_f64 * t6875 * t6867;
    (t21564, t21568, t21570, t21571, t21573, t21575)
}
