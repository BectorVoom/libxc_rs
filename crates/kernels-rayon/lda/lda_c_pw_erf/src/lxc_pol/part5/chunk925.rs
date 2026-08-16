//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 925/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk925(t1191: f64, t465: f64, t1138: f64, t1597: f64, t1578: f64, t2910: f64, t485: f64, t4259: f64, t142: f64, t450: f64, t1128: f64, t1159: f64, t285: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10810 = t1191 * t465;
    let t10812 = t10810 * t1138 * t1597;
    let t10816 = 0.03950778065781896_f64 * t1578 * t2910 * t485;
    let t10817 = 0.7561297733553868_f64 * t4259;
    let t10832 = t450 * t142;
    let t10868 = t1159 * t1128 * t285;
    (t10810, t10812, t10816, t10817, t10832, t10868)
}
