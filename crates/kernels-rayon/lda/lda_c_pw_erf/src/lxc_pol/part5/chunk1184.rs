//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1184/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1184(t2188: f64, t6988: f64, t1390: f64, t7792: f64, t1440: f64, t519: f64, t542: f64, t529: f64, t1325: f64, t494: f64, t2171: f64, t7004: f64) -> (f64, f64, f64, f64) {
    let t21530 = 8.0_f64 / 5.0_f64 * t6988 * t2188;
    let t21531 = t1390 * t7792;
    let t21535 = 4.0_f64 / 15.0_f64 * t519 * t1440 * t21531 * t542;
    let t21536 = t529 * t7792;
    let t21540 = 4.0_f64 / 15.0_f64 * t1325 * t1440 * t21536 * t494;
    let t21542 = 4.0_f64 / 5.0_f64 * t2171 * t7004;
    (t21530, t21535, t21540, t21542)
}
