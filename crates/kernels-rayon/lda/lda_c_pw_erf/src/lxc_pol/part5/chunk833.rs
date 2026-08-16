//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 833/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk833(t1243: f64, t7639: f64, t11: f64, t1966: f64, t2329: f64, t1245: f64, t7354: f64, t503: f64, t1971: f64, t504: f64, t7360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7640 = t1243 * t7639;
    let t7641 = t11 * t7640;
    let t7643 = t1966 * t2329;
    let t7644 = t1243 * t7643;
    let t7645 = t11 * t7644;
    let t7647 = t1245 * t7354;
    let t7648 = t503 * t7647;
    let t7649 = t11 * t7648;
    let t7651 = t1971 * t2329;
    let t7652 = t503 * t7651;
    let t7653 = t11 * t7652;
    let t7655 = t504 * t7360;
    (t7640, t7641, t7643, t7644, t7645, t7647, t7648, t7649, t7651, t7652, t7653, t7655)
}
