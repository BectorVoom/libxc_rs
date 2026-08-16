//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 586/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk586(t4420: f64, t721: f64, t4093: f64, t633: f64, t903: f64, t1106: f64, t3223: f64, t1040: f64, t1062: f64, t119: f64, t1098: f64, t1007: f64, t1067: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4421 = t4420 * t721;
    let t4426 = t903 * t4093 * t633;
    let t4429 = t1106 * t3223;
    let t4437 = t1040 * t1062;
    let t4438 = t4437 * t721;
    let t4440 = t1040 * t119;
    let t4445 = t1098 * t3223;
    let t4449 = t1007 * t1067;
    (t4421, t4426, t4429, t4438, t4440, t4445, t4449)
}
