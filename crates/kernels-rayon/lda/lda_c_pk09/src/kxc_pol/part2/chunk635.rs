//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 635/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk635(t1319: f64, t5308: f64, t1331: f64, t1336: f64, t1625: f64, t1311: f64, t1364: f64, t1434: f64, t1348: f64, t1215: f64, t309: f64, t1338: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5309 = t1319 * t5308;
    let t5311 = t1331 * t1336;
    let t5312 = t5311 * t1625;
    let t5316 = t1311 * t5308;
    let t5325 = t1434 * t1364;
    let t5326 = t1348 * t5325;
    let t5328 = t1215 * t309;
    let t5333 = t1434 * t1338;
    (t5309, t5312, t5316, t5326, t5328, t5333)
}
