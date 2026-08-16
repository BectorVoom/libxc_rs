//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 950/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk950(t409: f64, t9927: f64, t6041: f64, t5785: f64, t9578: f64, t15: f64, t309: f64, t9581: f64, t1494: f64, t2594: f64, t5777: f64, t1504: f64, t310: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10000 = t409 * t9927;
    let t10001 = t6041 * t10000;
    let t10003 = t9578 * t5785;
    let t10004 = t309 * t15;
    let t10005 = t10004 * t9581;
    let t10010 = t2594 * t1494;
    let t10011 = t10010 * t5777;
    let t10013 = t309 * t310 * t1504;
    (t10001, t10003, t10004, t10005, t10011, t10013)
}
