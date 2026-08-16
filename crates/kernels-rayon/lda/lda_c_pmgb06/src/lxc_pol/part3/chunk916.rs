//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 916/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk916(t1423: f64, t2966: f64, t1447: f64, t2877: f64, t3216: f64, t464: f64, t1387: f64, t3220: f64, t3260: f64, t3031: f64, t442: f64, t3248: f64, t517: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10393 = t1423 * t2966;
    let t10403 = t1447 * t2877;
    let t10412 = t3216 * t464;
    let t10416 = t3220 * t1387;
    let t10431 = t3260 * t464;
    let t10439 = t442 * t3031;
    let t10445 = t3248 * t517;
    (t10393, t10403, t10412, t10416, t10431, t10439, t10445)
}
