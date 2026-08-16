//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1005/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1005(t1408: f64, t1877: f64, t1915: f64, t22959: f64, t23290: f64, t25: f64, t25013: f64, t25015: f64, t25021: f64, t25024: f64, t25028: f64, t2522: f64, t25354: f64, t25358: f64, t25366: f64, t25372: f64, t25375: f64, t25377: f64, t25381: f64, t25385: f64, t25392: f64, t25397: f64, t606: f64, t6542: f64, t6666: f64, t6670: f64, t6671: f64, t7475: f64, t7541: f64, t7545: f64) -> f64 {
    let t25398 = 3.0_f64 * t25013 * t25015 + 3.0_f64 / 2.0_f64 * t2522 * t6666 * t7475 - 3.0_f64 / 2.0_f64 * t22959 * t25021 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t25024 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t25028 + 3.0_f64 / 2.0_f64 * t2522 * t7541 * t6542 + t1877 * t25354 * t25 / 2.0_f64 - t1877 * t25358 * t6671 / 2.0_f64 + t1877 * t7541 * t606 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t22959 * t25366 - t1877 * t23290 * t7545 / 2.0_f64 + t25372 * t25375 - t1877 * t6670 * t25377 / 2.0_f64 - t1877 * t6670 * t25381 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t1915 * t25385 + t1877 * t6666 * t1408 / 2.0_f64 - t1877 * t6670 * t25392 / 2.0_f64 + t25397;
    t25398
}
