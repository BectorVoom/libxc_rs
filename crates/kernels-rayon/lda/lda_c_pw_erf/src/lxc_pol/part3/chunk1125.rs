//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1125/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1125(t1318: f64, t1466: f64, t5029: f64, t549: f64, t581: f64, t1476: f64, t5334: f64, t12874: f64, t1469: f64, t1325: f64, t1390: f64, t1392: f64, t34: f64, t4956: f64) -> (f64, f64, f64, f64) {
    let t13162 = 4.0_f64 / 5.0_f64 * t1318 * t1466 * t581 * t5029 * t549;
    let t13163 = t5334 * t1476;
    let t13164 = 16.0_f64 / 45.0_f64 * t13163;
    let t13166 = 8.0_f64 / 5.0_f64 * t12874 * t1469;
    let t13171 = 8.0_f64 / 5.0_f64 * t1325 * t4956 * t1390 * t34 * t1392;
    (t13162, t13164, t13166, t13171)
}
