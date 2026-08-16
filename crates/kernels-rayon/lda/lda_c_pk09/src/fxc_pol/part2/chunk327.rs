//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 327/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk327(t296: f64, t343: f64, t1243: f64, t1255: f64, t1263: f64, t1272: f64, t1251: f64, t1259: f64, t1268: f64, t1275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1493 = t296 * t296;
    let t1494 = 1.0_f64 / t1493;
    let t1495 = t1494 * t343;
    let t1496 = 0.22687409291590604_f64 * t1243;
    let t1498 = 0.07562469763863536_f64 * t1255;
    let t1500 = 0.04525483399593904_f64 * t1263;
    let t1502 = 0.015084944665313014_f64 * t1272;
    let t1504 = t1496 - 0.22687409291590604_f64 * t1251 + t1498 + 0.22687409291590604_f64 * t1259 + t1500 - 0.04525483399593904_f64 * t1268 + t1502 + 0.04525483399593904_f64 * t1275;
    (t1493, t1494, t1495, t1496, t1498, t1500, t1502, t1504)
}
