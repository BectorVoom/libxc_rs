//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1024/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1024(t2470: f64, t5194: f64, t1423: f64, t7577: f64, t7581: f64, t16343: f64, t806: f64, t2007: f64, t6127: f64, t1980: f64, t2012: f64, t2591: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19251 = t5194 * t2470;
    let t19252 = 2.0_f64 / 27.0_f64 * t19251;
    let t19253 = t1423 * t7577;
    let t19254 = 2.0_f64 / 27.0_f64 * t19253;
    let t19255 = t1423 * t7581;
    let t19256 = 2.0_f64 / 135.0_f64 * t19255;
    let t19258 = t16343 * t806 / 15.0_f64;
    let t19260 = t6127 * t2007 / 15.0_f64;
    let t19263 = 2.0_f64 / 15.0_f64 * t2591 * t1980 * t2012;
    (t19252, t19254, t19256, t19258, t19260, t19263)
}
