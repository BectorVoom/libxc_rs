//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1030/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1030(t1983: f64, t23857: f64, t8640: f64, t115271: f64, t115275: f64, t115277: f64, t115279: f64, t115283: f64, t115666: f64, t115669: f64, t115672: f64, t115674: f64, t115676: f64, t115678: f64, t115681: f64, t1976: f64, t23951: f64, t24008: f64, t24176: f64, t31246: f64, t7171: f64, t8329: f64, t8450: f64) -> f64 {
    let t115684 = 2.0_f64 * t1983 * t8640 * t23857;
    let t115685 = -t1976 * t24008 - t23951 * t8450 + 6.0_f64 * t24176 * t8450 + 6.0_f64 * t31246 * t7171 - t115271 - t115275 - t115277 - t115279 + t115283 + t115666 - t115669 - t115672 - t115674 - t115676 + t115678 + t115681 + t115684 - t8329;
    t115685
}
