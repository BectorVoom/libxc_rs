//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 395/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk395<F: Float>(t1498: F, t221: F, t1239: F, t1241: F, t1248: F, t1254: F, t1258: F, t173: F, t184: F) -> (F, F, F, F, F) {
    let t1500 = 2.0 / 15.0 * t1498 * t221;
    let t1501 = 0.002518888888888889 * t1239;
    let t1506 = -t1501 - 0.0012594444444444445 * t1241 + 0.0012594444444444445 * t1248 - 0.003778333333333333 * t1254 + 0.0018891666666666666 * t1258;
    let t1507 = t173 * t1506;
    let t1508 = t1507 * t184;
    (t1500, t1501, t1506, t1507, t1508)
}
