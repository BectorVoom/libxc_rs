//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 582/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk582<F: Float>(t1319: F, t5308: F, t1331: F, t1336: F, t1625: F, t1311: F, t1364: F, t1434: F, t1348: F, t1215: F, t309: F, t1338: F, t1337: F, t131: F, t1350: F, t1369: F, t4998: F) -> (F, F, F, F, F, F, F, F) {
    let t5309 = t1319 * t5308;
    let t5311 = t1331 * t1336;
    let t5312 = t5311 * t1625;
    let t5316 = t1311 * t5308;
    let t5325 = t1434 * t1364;
    let t5326 = t1348 * t5325;
    let t5328 = t1215 * t309;
    let t5333 = t1434 * t1338;
    let t5335 = 0.027433775686566395 * t1337 * t5333;
    let t5336 = t131 * t1350;
    let t5337 = t1348 * t5336;
    let t5340 = 12.423505345088643 * t1369 * t4998;
    (t5309, t5312, t5316, t5326, t5328, t5335, t5337, t5340)
}
