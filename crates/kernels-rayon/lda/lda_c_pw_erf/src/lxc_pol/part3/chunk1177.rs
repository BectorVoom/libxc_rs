//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1177/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1177(t13852: f64, t13872: f64, t173: f64, t184: f64, t199: f64, t1405: f64, t1960: f64, t13796: f64, t13800: f64, t13802: f64, t13807: f64, t13811: f64, t13816: f64, t13819: f64, t13821: f64, t13824: f64, t13827: f64, t13832: f64) -> (f64, f64, f64) {
    let t13877 = 2.0_f64 / 15.0_f64 * t173 * (t13852 + t13872) * t184 * t199;
    let t13879 = 4.0_f64 / 5.0_f64 * t1960 * t1405;
    let t13880 = t13796 + t13800 + t13802 + t13807 + t13811 + t13816 - t13819 - t13821 - t13824 - t13827 - t13832 + t13877 + t13879;
    (t13877, t13879, t13880)
}
