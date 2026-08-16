//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1120/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1120(t3255: f64, t4634: f64, t330: f64, t4670: f64, t829: f64, t3274: f64, t2635: f64, t4632: f64, t1727: f64, t2844: f64, t2630: f64, t10297: f64) -> (f64, f64, f64, f64) {
    let t14137 = t3255 * t4634;
    let t14139 = t4670 * t330;
    let t14140 = t14139 * t829;
    let t14141 = t3274 * t14140;
    let t14144 = t4632 * t2635;
    let t14145 = t3274 * t14144;
    let t14148 = t1727 * t2844;
    let t14149 = t14148 * t2630;
    let t14150 = t10297 * t14149;
    (t14137, t14141, t14145, t14150)
}
