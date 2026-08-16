//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 812/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk812(t415: f64, t763: f64, t5594: f64, t1664: f64, t1852: f64, t10: f64, t127: f64, t1568: f64, t3313: f64, t3322: f64, t426: f64, t5588: f64, t5591: f64, t5596: f64, t5598: f64, t5599: f64, t5603: f64) -> (f64, f64, f64, f64) {
    let t5607 = t415 * t763;
    let t5609 = 1.9486833333333333_f64 * t5607 * t5594;
    let t5610 = t1852 * t1664;
    let t5614 = 5.87616_f64 * t127 * t1852 * t1568 + t5588 + t5591 - t5596 - t5598 + 3.0_f64 * t426 * t10 * t5599 + 3.0_f64 / 2.0_f64 * t426 * t10 * t5603 - t5609 - 6.0_f64 * t426 * t10 * t5610 + t3313 - t3322;
    (t5607, t5609, t5610, t5614)
}
