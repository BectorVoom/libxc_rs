//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1166/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1166<F: Float>(t12406: F, t15323: F, t15324: F, t12397: F, t12402: F, t12325: F, t12329: F, t12337: F, t12354: F, t12356: F, t9147: F, t9179: F, t9181: F, t9215: F) -> (F, F, F, F) {
    let t15326 = t15323 * t12406 * t15324;
    let t15329 = t15323 * t12397 * t15324;
    let t15332 = t15323 * t12402 * t15324;
    let t15338 = -F::cast_from(0.0016792592592592592_f64) * t9147 + F::cast_from(0.0008396296296296296_f64) * t9179 + F::cast_from(0.000559753086419753_f64) * t9181 - F::cast_from(0.003918271604938271_f64) * t9215 - F::cast_from(0.002518888888888889_f64) * t12325 - F::cast_from(0.06045333333333333_f64) * t15326 + F::cast_from(0.01679259259259259_f64) * t15329 + F::new(0.09068) * t15332 + F::cast_from(0.059613703703703703_f64) * t12329 - F::cast_from(0.003918271604938271_f64) * t12337 - F::cast_from(0.0033585185185185185_f64) * t12354 + F::cast_from(0.002518888888888889_f64) * t12356;
    (t15326, t15329, t15332, t15338)
}
