//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1085/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1085(t12693: f64, t197: f64, t4906: f64, t1325: f64, t5417: f64, t1326: f64, t2954: f64, t5264: f64, t1318: f64, t1319: f64, t2967: f64, t5229: f64) -> (f64, f64, f64, f64, f64) {
    let t12694 = 32.0_f64 / 45.0_f64 * t12693;
    let t12695 = t4906 * t197;
    let t12697 = t1325 * t12695 * t5417;
    let t12698 = 8.0_f64 / 9.0_f64 * t12697;
    let t12702 = 16.0_f64 / 15.0_f64 * t1325 * t1326 * t5264 * t2954;
    let t12706 = 16.0_f64 / 15.0_f64 * t1318 * t1319 * t5229 * t2967;
    (t12694, t12695, t12698, t12702, t12706)
}
