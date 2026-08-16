//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1022/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1022(t1287: f64, t1318: f64, t5269: f64, t593: f64, t833: f64, t1381: f64, t5270: f64, t1466: f64, t3667: f64, t571: f64, t1401: f64, t3899: f64) -> (f64, f64, f64, f64, f64) {
    let t11978 = 8.0_f64 / 5.0_f64 * t1318 * t5269 * t833 * t1287 * t593;
    let t11982 = 8.0_f64 / 5.0_f64 * t1318 * t5269 * t5270 * t1381;
    let t11983 = t1466 * t3667;
    let t11984 = t833 * t593;
    let t11988 = 12.0_f64 / 5.0_f64 * t571 * t11983 * t11984 * t1381;
    let t11989 = t3899 * t1401;
    (t11978, t11982, t11983, t11988, t11989)
}
