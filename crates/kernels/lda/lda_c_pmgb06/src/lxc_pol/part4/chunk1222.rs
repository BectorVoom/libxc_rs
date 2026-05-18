//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1222/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1222<F: Float>(t16106: F, t1444: F, t6131: F, t16083: F, t16087: F, t16090: F, t16092: F, t16094: F, t16095: F, t16099: F, t16100: F, t16101: F, t16102: F, t16103: F, t16105: F) -> (F, F, F) {
    let t16107 = F::new(4.0) / F::new(405.0) * t16106;
    let t16109 = F::new(2.0) / F::new(45.0) * t1444 * t6131;
    let t16110 = -t16083 - t16087 + t16090 - t16092 - t16094 - t16095 - t16099 - t16100 + t16101 - t16102 - t16103 - t16105 - t16107 + t16109;
    (t16107, t16109, t16110)
}
