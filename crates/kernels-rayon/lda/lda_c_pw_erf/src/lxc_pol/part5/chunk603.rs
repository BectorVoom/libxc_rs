//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 603/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk603(t159: f64, t285: f64, t4137: f64, t169: f64, t274: f64, t2817: f64, t301: f64, t1131: f64, t1586: f64, t485: f64, t1138: f64, t1597: f64, t2881: f64) -> (f64, f64, f64, f64) {
    let t4140 = 0.006715335817467199_f64 * t4137 * t159 * t285;
    let t4144 = 0.9247854820715865_f64 * t169 * t2817 * t274 * t301;
    let t4153 = t1586 * t1131 * t485;
    let t4156 = t2881 * t1138 * t1597;
    (t4140, t4144, t4153, t4156)
}
