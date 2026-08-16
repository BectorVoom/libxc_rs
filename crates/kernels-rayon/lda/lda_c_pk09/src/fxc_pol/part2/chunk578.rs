//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 578/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk578(t4110: f64, t721: f64, t1006: f64, t119: f64, t10: f64, t1005: f64, t88: f64, t1063: f64, t3223: f64, t3089: f64, t3101: f64, t3119: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4111 = t4110 * t721;
    let t4113 = t1006 * t119;
    let t4119 = t1005 * t88 * t10;
    let t4123 = t1063 * t3223;
    let t4125 = 0.14975624337724558_f64 * t3089;
    let t4128 = 0.01233429741534199_f64 * t3101;
    let t4134 = 0.19967499116966075_f64 * t3119;
    (t4111, t4113, t4119, t4123, t4125, t4128, t4134)
}
