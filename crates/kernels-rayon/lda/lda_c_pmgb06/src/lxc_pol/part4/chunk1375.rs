//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1375/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1375(t2414: f64, t740: f64, t1193: f64, t1354: f64, t6716: f64, t81: f64, t118: f64, t415: f64, t6946: f64, t15116: f64, t3: f64, t6928: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18057 = t740 * t2414;
    let t18059 = t18057 * t1193 * t1354;
    let t18061 = t81 * t6716;
    let t18062 = t18061 * t118;
    let t18064 = t6946 * t415;
    let t18066 = t3 * t15116;
    let t18069 = t6928 * t415;
    (t18057, t18059, t18061, t18062, t18064, t18066, t18069)
}
