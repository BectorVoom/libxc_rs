//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 999/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk999(t2171: f64, t5339: f64, t1318: f64, t2531: f64, t9432: f64, t518: f64, t6596: f64, t6600: f64, t1450: f64, t6198: f64, t1390: f64, t6566: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15570 = t2171 * t5339;
    let t15573 = t1318 * t9432 * t2531;
    let t15579 = t6596 * t518;
    let t15582 = t6600 * t518;
    let t15587 = t6198 * t1450;
    let t15590 = t1390 * t6566;
    (t15570, t15573, t15579, t15582, t15587, t15590)
}
