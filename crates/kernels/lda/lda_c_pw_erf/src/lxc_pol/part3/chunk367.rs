//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 367/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk367<F: Float>(t1352: F, t1371: F, t1356: F, t589: F, t1360: F, t1346: F, t1347: F, t1354: F, t1358: F, t1362: F, t1366: F, t1367: F, t25: F) -> (F, F, F, F) {
    let t1372 = t1371 * t1352;
    let t1375 = t589 * t1356;
    let t1378 = t589 * t1360;
    let t1381 = t1346 + 0.023994444444444443 * t1347 - 0.023994444444444443 * t1354 + 0.07198333333333333 * t1358 - 0.035991666666666665 * t1362 + t1366 + 0.008888888888888889 * t1367 - 0.0022222222222222222 * t25 * t1372 + 0.013333333333333334 * t25 * t1375 - 0.006666666666666667 * t25 * t1378;
    (t1372, t1375, t1378, t1381)
}
