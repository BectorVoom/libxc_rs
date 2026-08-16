//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1110/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1110(t2070: f64, t493: f64, t785: f64, t11898: f64, t2130: f64, t1318: f64, t3899: f64, t4942: f64, t1466: f64, t3667: f64, t3669: f64, t811: f64) -> (f64, f64, f64, f64) {
    let t12984 = t493 * t2070 * t785;
    let t12985 = 32.0_f64 / 405.0_f64 * t12984;
    let t12987 = t493 * t11898 * t2130;
    let t12988 = 64.0_f64 / 45.0_f64 * t12987;
    let t12990 = t1318 * t3899 * t4942;
    let t12991 = 16.0_f64 / 15.0_f64 * t12990;
    let t12996 = 8.0_f64 / 5.0_f64 * t1318 * t1466 * t3667 * t811 * t3669;
    (t12985, t12988, t12991, t12996)
}
