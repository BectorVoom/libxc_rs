//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 685/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk685(t1934: f64, t6477: f64, t1672: f64, t1944: f64, t1931: f64, t6292: f64, t1468: f64, t447: f64, t1747: f64, t6302: f64, t1222: f64, t1799: f64) -> (f64, f64, f64, f64, f64) {
    let t6478 = t1934 * t6477;
    let t6480 = t1944 * t1672;
    let t6483 = 4.937333717448355_f64 * t1931 * t6292;
    let t6484 = t447 * t1468;
    let t6485 = t6484 * t1747;
    let t6487 = 38.978347549160304_f64 * t6485 * t6302;
    let t6488 = t1222 * t1799;
    (t6478, t6480, t6483, t6487, t6488)
}
