//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 967/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk967(t13478: f64, t1318: f64, t2157: f64, t9432: f64, t1472: f64, t5342: f64, t1184: f64, t2152: f64, t571: f64, t573: f64, t1446: f64, t5339: f64) -> (f64, f64, f64, f64, f64) {
    let t13479 = 8.0_f64 / 135.0_f64 * t13478;
    let t13507 = t1318 * t9432 * t2157;
    let t13508 = 8.0_f64 / 45.0_f64 * t13507;
    let t13511 = t1472 * t5342;
    let t13512 = 8.0_f64 / 135.0_f64 * t13511;
    let t13515 = t571 * t1184 * t573 * t2152;
    let t13517 = t1446 * t5339;
    (t13479, t13508, t13512, t13515, t13517)
}
