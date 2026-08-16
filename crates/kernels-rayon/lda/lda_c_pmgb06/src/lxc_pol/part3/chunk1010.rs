//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1010/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1010(t517: f64, t5312: f64, t1381: f64, t493: f64, t1382: f64, t5305: f64, t1972: f64, t2980: f64, t1420: f64, t5483: f64, t1423: f64, t5345: f64) -> (f64, f64, f64, f64, f64) {
    let t12012 = t5312 * t517;
    let t12015 = 2.0_f64 / 15.0_f64 * t493 * t12012 * t1381;
    let t12017 = 2.0_f64 / 15.0_f64 * t5305 * t1382;
    let t12019 = 2.0_f64 / 15.0_f64 * t1972 * t2980;
    let t12021 = 2.0_f64 / 15.0_f64 * t1420 * t5483;
    let t12022 = t1423 * t5345;
    (t12015, t12017, t12019, t12021, t12022)
}
