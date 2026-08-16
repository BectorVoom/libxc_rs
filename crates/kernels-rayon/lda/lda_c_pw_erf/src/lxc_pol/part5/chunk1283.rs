//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1283/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1283(t1991: f64, t22764: f64, t519: f64, t1325: f64, t494: f64, t5250: f64, t7635: f64, t542: f64, t9700: f64, t14200: f64, t22713: f64, t14205: f64, t22717: f64) -> (f64, f64, f64, f64, f64) {
    let t22967 = 4.0_f64 / 27.0_f64 * t519 * t1991 * t22764;
    let t22971 = 64.0_f64 / 81.0_f64 * t1325 * t5250 * t7635 * t494;
    let t22975 = 32.0_f64 / 81.0_f64 * t519 * t9700 * t7635 * t542;
    let t22978 = 352.0_f64 / 243.0_f64 * t519 * t14200 * t22713;
    let t22981 = 64.0_f64 / 27.0_f64 * t519 * t14205 * t22717;
    (t22967, t22971, t22975, t22978, t22981)
}
