//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1166/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1166(t12406: f64, t15323: f64, t15324: f64, t12397: f64, t12402: f64, t12325: f64, t12329: f64, t12337: f64, t12354: f64, t12356: f64, t9147: f64, t9179: f64, t9181: f64, t9215: f64) -> (f64, f64, f64, f64) {
    let t15326 = t15323 * t12406 * t15324;
    let t15329 = t15323 * t12397 * t15324;
    let t15332 = t15323 * t12402 * t15324;
    let t15338 = -0.0016792592592592592_f64 * t9147 + 0.0008396296296296296_f64 * t9179 + 0.000559753086419753_f64 * t9181 - 0.003918271604938271_f64 * t9215 - 0.002518888888888889_f64 * t12325 - 0.06045333333333333_f64 * t15326 + 0.01679259259259259_f64 * t15329 + 0.09068_f64 * t15332 + 0.059613703703703703_f64 * t12329 - 0.003918271604938271_f64 * t12337 - 0.0033585185185185185_f64 * t12354 + 0.002518888888888889_f64 * t12356;
    (t15326, t15329, t15332, t15338)
}
