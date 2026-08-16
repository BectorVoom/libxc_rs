//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 914/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk914(t10318: f64, t446: f64, t2010: f64, t1447: f64, t3285: f64, t1981: f64, t500: f64, t1417: f64, t3226: f64, t3223: f64, t1166: f64, t1696: f64, t208: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10319 = t10318 * t446;
    let t10321 = t2010 * t446;
    let t10333 = t1447 * t3285;
    let t10335 = t1981 * t500;
    let t10337 = t3226 * t1417;
    let t10339 = t3223 * t1417;
    let t10343 = t1166 * t1696 * t208 * t213;
    (t10319, t10321, t10333, t10335, t10337, t10339, t10343)
}
