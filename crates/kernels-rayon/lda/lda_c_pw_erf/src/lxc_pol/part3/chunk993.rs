//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 993/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk993(t10823: f64, t10849: f64, t11557: f64, t11561: f64, t11563: f64, t11568: f64, t11570: f64, t11574: f64, t11577: f64, t11588: f64, t11597: f64, t1550: f64, t1733: f64, t1881: f64, t2211: f64, t2764: f64, t2767: f64, t2799: f64, t4117: f64, t4441: f64, t5670: f64, t777: f64, t9127: f64) -> f64 {
    let t11599 = 0.19513566535229734_f64 * t11557 + 0.0001639671923854359_f64 * t11561 - 0.15965645347006147_f64 * t11563 + t11568 - 9.0_f64 * t2764 * t11570 - 18.0_f64 * t11574 * t2767 - 6.0_f64 * t2764 * t11577 + 6.0_f64 * t4117 * t4441 + 3.0_f64 * t2211 * t10823 + 9.0_f64 * t2211 * t10849 - 3.0_f64 * t1881 * t2799 + 3.0_f64 * t1733 * t11588 - 3.0_f64 * t777 * t9127 + 3.0_f64 * t5670 * t1550 - 0.9247854820715865_f64 * t11597;
    t11599
}
