//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1338/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1338(t105762: f64, t22960: f64, t28248: f64, t86721: f64, t1530: f64, t5660: f64, t25373: f64, t193: f64, t20756: f64, t105727: f64, t105732: f64, t105741: f64, t105745: f64, t105755: f64, t105759: f64, t1877: f64, t1915: f64, t1916: f64, t20216: f64, t22959: f64, t25: f64, t25013: f64, t2522: f64, t25372: f64, t28241: f64, t28249: f64, t4314: f64, t5397: f64, t7541: f64, t7545: f64, t86736: f64, t98054: f64) -> (f64, f64, f64) {
    let t105763 = t22960 * t105762;
    let t105766 = t86721 * t28248;
    let t105769 = t1530 * t5660;
    let t105770 = t25373 * t105769;
    let t105773 = t193 * t20756;
    let t105776 = -3.0_f64 / 2.0_f64 * t1877 * t98054 * t7545 + t1877 * t105727 * t25 / 2.0_f64 + 9.0_f64 * t22959 * t105732 + 9.0_f64 * t4314 * t7541 * t28241 + 3.0_f64 / 2.0_f64 * t1877 * t7541 * t5397 + 9.0_f64 / 2.0_f64 * t2522 * t1915 * t105741 + 9.0_f64 / 2.0_f64 * t2522 * t1915 * t105745 + t1877 * t1915 * t20216 / 2.0_f64 - 9.0_f64 * t86736 * t28249 - 9.0_f64 / 2.0_f64 * t22959 * t105755 - 9.0_f64 / 2.0_f64 * t22959 * t105759 - 9.0_f64 * t25013 * t105763 - 9.0_f64 * t22959 * t105766 + 3.0_f64 * t25372 * t105770 + 3.0_f64 * t105773 * t1916;
    (t105769, t105773, t105776)
}
