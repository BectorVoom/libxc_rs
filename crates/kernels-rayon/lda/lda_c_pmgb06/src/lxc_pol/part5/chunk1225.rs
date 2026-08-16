//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1225/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1225(t12306: f64, t12304: f64, t12311: f64, t12313: f64, t18225: f64, t19971: f64, t19972: f64, t19975: f64, t19976: f64, t19977: f64, t19978: f64, t19979: f64) -> f64 {
    let t21942 = 1.2e-20_f64 * t12306;
    let t21944 = -t19971 - t19972 - t19975 - t19976 + 2.0_f64 * t12304 + t21942 + t12311 + t12313 - t19977 + t19978 - 2.0_f64 / 9.0_f64 * t18225 - t19979;
    t21944
}
