//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 783/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk783(t7276: f64, t7325: f64, t2722: f64, t787: f64, t2730: f64, t2448: f64, t769: f64, t2247: f64, t2248: f64, t3505: f64, t3517: f64, t3525: f64, t3643: f64, t5852: f64, t69: f64, t7069: f64, t7071: f64, t7261: f64, t7262: f64, t7270: f64, t7271: f64, t7274: f64, t7283: f64, t7309: f64, t7318: f64, t7322: f64) -> (f64, f64, f64, f64, f64) {
    let t7326 = t7276 + t7325;
    let t7334 = t2722 * t787;
    let t7337 = t787 * t2730;
    let t7344 = t769 * t2448;
    let t7351 = -5.172765_f64 * t7069 + 1.724255_f64 * t7071 - 20.69106_f64 * t69 * t7322 + 15.518295_f64 * t2247 * t2248 * t7344 - t7261 + t7262 - t7283 + t7270 - 1.724255_f64 * t69 * t7318 - t7309 - 2.2990066666666666_f64 * t5852 + t7274 - t7271 - t3643 - t3505 - t3517 + t3525;
    (t7326, t7334, t7337, t7344, t7351)
}
