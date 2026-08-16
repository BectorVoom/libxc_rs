//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 980/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk980(t1020: f64, t10556: f64, t1058: f64, t10592: f64, t10593: f64, t10594: f64, t10731: f64, t135: f64, t144: f64, t1535: f64, t2536: f64, t4996: f64, t5005: f64, t5011: f64, t5019: f64, t5022: f64, t5178: f64, t5186: f64, t560: f64, t639: f64, t8751: f64, t9112: f64, t9121: f64) -> f64 {
    let t10747 = t10731 * t135 * t144 * t639 + 9.0_f64 * t1020 * t1535 * t9112 - 9.0_f64 * t1020 * t1535 * t9121 + 3.0_f64 * t10556 * t135 * t560 - 3.0_f64 * t1058 * t2536 * t8751 - t10592 - t10593 + t10594 + t4996 + t5005 - t5011 + t5019 - t5022 + t5178 + t5186;
    t10747
}
