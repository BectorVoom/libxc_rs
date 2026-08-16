//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1016/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1016(t19130: f64, t3: f64, t415: f64, t7874: f64, t10792: f64, t10795: f64, t10802: f64, t10806: f64, t10808: f64, t10811: f64, t10813: f64, t10817: f64, t10825: f64, t10828: f64, t118: f64, t14501: f64, t15163: f64, t15166: f64, t19126: f64) -> (f64, f64) {
    let t19131 = t3 * t19130;
    let t19134 = t7874 * t415;
    let t19140 = 0.031505407223141116_f64 * t19126 - 0.005926167098672845_f64 * t15163 - 0.01185233419734569_f64 * t15166 - 0.031505407223141116_f64 * t19131 * t118 - 0.031505407223141116_f64 * t19134 - t10792 - t14501 + 0.0034679929861433484_f64 * t10795 - 0.0014862827083471494_f64 * t10802 - t10806 - t10808 - t10811 - 0.005926167098672845_f64 * t10813 + t10817 - 0.025899545097903542_f64 * t10825 - t10828;
    (t19131, t19140)
}
