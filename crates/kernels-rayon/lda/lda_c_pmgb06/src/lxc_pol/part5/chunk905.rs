//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 905/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk905(t10970: f64, t317: f64, t321: f64, t4001: f64, t934: f64, t97: f64, t1786: f64, t27: f64, t2767: f64, t927: f64, t2368: f64, t754: f64, t936: f64) -> (f64, f64, f64) {
    let t10976 = 0.3407285805772476_f64 * t4001 * t321 / t10970 * t317 * t97 * t934;
    let t10980 = t927 * t1786 * t27 * t2767;
    let t10984 = t2368 * t754 * t97 * t936;
    (t10976, t10980, t10984)
}
