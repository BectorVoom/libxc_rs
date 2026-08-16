//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 857/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk857(t248: f64, t3890: f64, t653: f64, t1024: f64, t3697: f64, t634: f64, t3963: f64, t3969: f64, t3957: f64, t110: f64, t1121: f64, t3711: f64) -> (f64, f64, f64, f64, f64) {
    let t8548 = t248 * t653 * t3890;
    let t8552 = 8.0_f64 * t1024 * t634 * t3697;
    let t8553 = t3969 * t3963;
    let t8555 = t3969 * t3957;
    let t8559 = 3.8527786510141255_f64 * t1121 * t110 * t3711;
    (t8548, t8552, t8553, t8555, t8559)
}
