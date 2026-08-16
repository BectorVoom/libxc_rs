//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 622/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk622(t3559: f64, t64: f64, t35: f64, t1227: f64, t3494: f64, t3505: f64, t3508: f64, t3513: f64, t3515: f64, t3517: f64, t3521: f64, t3523: f64, t3525: f64, t3526: f64, t3531: f64, t3534: f64, t360: f64, t63: f64) -> (f64, f64, f64) {
    let t3560 = t64 * t3559;
    let t3561 = t35 * t3560;
    let t3564 = 17.62848_f64 * t63 * t3494 * t1227 - t3505 + t3508 + t3513 - t3515 - t3517 - t3521 - t3523 + t3525 + 9.0_f64 / 2.0_f64 * t360 * t35 * t3526 - 2.0_f64 / 3.0_f64 * t3531 + t3534 / 2.0_f64 - t360 * t3561 / 2.0_f64;
    (t3560, t3561, t3564)
}
