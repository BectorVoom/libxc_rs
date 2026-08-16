//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 949/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk949(t2414: f64, t539: f64, t188: f64, t4723: f64, t4725: f64, t6229: f64, t6234: f64, t6236: f64, t6238: f64, t6240: f64, t6243: f64, t6247: f64, t6249: f64, t6252: f64, t6257: f64, t6261: f64, t6265: f64) -> (f64, f64) {
    let t7179 = t2414 * t539;
    let t7180 = t7179 * t188;
    let t7182 = -t4723 + t4725 + 4.0_f64 / 3.0_f64 * t7180 - t6229 - t6234 - t6236 - t6238 + t6240 + t6243 + t6247 + t6249 + t6252 - t6257 + t6261 + t6265;
    (t7179, t7182)
}
