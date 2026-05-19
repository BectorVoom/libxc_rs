//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 402/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk402<F: Float>(t1484: F, t22: F, t1351: F, t219: F, t951: F, t571: F, t1345: F, t1347: F, t1354: F, t1358: F, t1362: F, t203: F) -> (F, F, F, F, F, F, F, F) {
    let t1485 = t22 * t1484;
    let t1486 = t219 * t1351;
    let t1487 = t1486 * t951;
    let t1488 = t1485 * t1487;
    let t1490 = F::new(4.0) / F::new(27.0) * t571 * t1488;
    let t1491 = F::cast_from(0.002518888888888889_f64) * t1345;
    let t1496 = -t1491 - F::cast_from(0.0012594444444444445_f64) * t1347 + F::cast_from(0.0012594444444444445_f64) * t1354 - F::cast_from(0.003778333333333333_f64) * t1358 + F::cast_from(0.0018891666666666666_f64) * t1362;
    let t1497 = t203 * t1496;
    (t1485, t1486, t1487, t1488, t1490, t1491, t1496, t1497)
}
