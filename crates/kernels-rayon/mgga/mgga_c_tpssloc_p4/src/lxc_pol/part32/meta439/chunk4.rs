//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1687/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1687(t3919: f64, t6330: f64, t12116: f64, t12118: f64, t12123: f64, t12130: f64, t12133: f64, t12141: f64, t15976: f64, t16171: f64, t19689: f64, t19690: f64, t19691: f64, t19693: f64, t19694: f64, t19695: f64, t19696: f64, t19697: f64, t19698: f64, t5126: f64, t9853: f64, t9859: f64) -> f64 {
    let t20093 = t3919 * t6330;
    let t20096 = 6.0_f64 * t20093 * t5126 + t12116 + t12118 + t12123 + t12130 + t12133 - t12141 - t15976 - t16171 + t19689 + t19690 - t19691 + t19693 + t19694 - t19695 + t19696 + t19697 - t19698 + t9853 + t9859;
    t20096
}
