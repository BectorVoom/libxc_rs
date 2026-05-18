//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 635/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk635<F: Float>(t1319: F, t5308: F, t1331: F, t1336: F, t1625: F, t1311: F, t1364: F, t1434: F, t1348: F, t1215: F, t309: F, t1338: F) -> (F, F, F, F, F, F) {
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
