//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 685/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk685<F: Float>(t1934: F, t6477: F, t1672: F, t1944: F, t1931: F, t6292: F, t1468: F, t447: F, t1747: F, t6302: F, t1222: F, t1799: F) -> (F, F, F, F, F) {
    let t6478 = t1934 * t6477;
    let t6480 = t1944 * t1672;
    let t6483 = F::cast_from(4.937333717448355_f64) * t1931 * t6292;
    let t6484 = t447 * t1468;
    let t6485 = t6484 * t1747;
    let t6487 = F::cast_from(38.978347549160304_f64) * t6485 * t6302;
    let t6488 = t1222 * t1799;
    (t6478, t6480, t6483, t6487, t6488)
}
