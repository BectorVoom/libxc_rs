//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 601/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk601(t1156: f64, t266: f64, t1161: f64, t1179: f64, t1185: f64, t258: f64, t4787: f64, t4789: f64, t4793: f64, t272: f64, t1171: f64, t256: f64) -> (f64, f64, f64, f64, f64) {
    let t4821 = t1156 * t266;
    let t4822 = t1179 * t1161;
    let t4823 = t4822 * t1185;
    let t4824 = t4821 * t4823;
    let t4830 = 1.8073681049360268_f64 * t4787 + 15.112064760386344_f64 * t4789 - 12.010155044502033_f64 * t258 + 0.5833333333333334_f64 * t4793;
    let t4831 = t272 * t4830;
    let t4833 = 1.28_f64 * t1156 * t4831;
    let t4837 = t256 * t1171;
    (t4821, t4824, t4830, t4833, t4837)
}
