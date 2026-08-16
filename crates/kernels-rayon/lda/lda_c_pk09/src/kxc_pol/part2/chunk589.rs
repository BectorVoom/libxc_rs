//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 589/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk589(t1092: f64, t4502: f64, t3148: f64, t1106: f64, t4365: f64, t10: f64, t125: f64, t86: f64, t975: f64, t143: f64, t3557: f64, t161: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4503 = t1092 * t4502;
    let t4504 = t4503 * t3148;
    let t4512 = t1106 * t4365;
    let t4517 = t86 * t125 * t10;
    let t4518 = t975 * t4517;
    let t4519 = t4518 * t3148;
    let t4528 = 4.178971354861182_f64 * t143 * t3557;
    let t4530 = 5.485926352720394_f64 * t161 * t3557;
    (t4504, t4512, t4517, t4519, t4528, t4530)
}
