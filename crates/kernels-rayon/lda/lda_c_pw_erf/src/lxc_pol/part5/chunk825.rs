//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 825/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk825(t1466: f64, t7569: f64, t1318: f64, t4734: f64, t7545: f64, t7547: f64, t7548: f64, t7549: f64, t7550: f64, t7551: f64, t7552: f64, t7553: f64, t7554: f64, t7556: f64, t7560: f64, t7562: f64, t7564: f64, t7566: f64, t7568: f64) -> (f64, f64, f64) {
    let t7570 = t1466 * t7569;
    let t7572 = 4.0_f64 / 5.0_f64 * t1318 * t7570;
    let t7573 = t7545 + 8.0_f64 * t4734 - t7547 - t7548 - t7549 + t7550 + t7551 + t7552 + t7553 + t7554 + t7556 + t7560 + t7562 + t7564 + t7566 - t7568 - t7572;
    (t7570, t7572, t7573)
}
