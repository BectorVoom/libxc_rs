//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 332/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk332<F: Float>(t1366: F, t543: F, t27: F, t534: F, t545: F, t540: F, t184: F, t97: F, t125: F, t186: F, t934: F) -> (F, F, F, F, F, F, F) {
    let t1368 = 0.07214027574909895 * t543 * t1366;
    let t1369 = t534 * t27;
    let t1370 = t1369 * t545;
    let t1372 = t540 * t27;
    let t1374 = 0.21642082724729686 * t1372 * t545;
    let t1375 = t184 * t97;
    let t1377 = t934 * t125 * t186;
    (t1368, t1369, t1370, t1372, t1374, t1375, t1377)
}
