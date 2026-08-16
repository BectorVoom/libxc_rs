//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 753/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk753(t1446: f64, t1987: f64, t1326: f64, t4637: f64, t519: f64, t1991: f64, t4615: f64, t4633: f64, t4829: f64, t1992: f64, t1484: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4855 = 16.0_f64 / 45.0_f64 * t1446 * t1987;
    let t4856 = t1326 * t4637;
    let t4858 = 8.0_f64 / 45.0_f64 * t519 * t4856;
    let t4859 = t1991 * t4615;
    let t4861 = 8.0_f64 / 9.0_f64 * t519 * t4859;
    let t4862 = t4829 * t4633;
    let t4864 = 32.0_f64 / 45.0_f64 * t519 * t4862;
    let t4866 = 8.0_f64 / 27.0_f64 * t1446 * t1992;
    let t4867 = t473 * t1484;
    (t4855, t4856, t4858, t4859, t4861, t4862, t4864, t4866, t4867)
}
