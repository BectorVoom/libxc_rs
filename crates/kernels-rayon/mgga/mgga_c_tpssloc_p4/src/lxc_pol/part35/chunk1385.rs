//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1385/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1385(t105776: f64, t105829: f64, t20217: f64, t3: f64, t105726: f64, t105731: f64, t105754: f64, t105758: f64, t105762: f64, t105769: f64, t1484: f64, t1530: f64, t1877: f64, t1914: f64, t1915: f64, t193: f64, t202: f64, t20756: f64, t20778: f64, t20800: f64, t20947: f64, t21066: f64, t23295: f64, t2522: f64, t25358: f64, t28248: f64, t28448: f64, t4314: f64, t5527: f64, t5544: f64, t5660: f64, t5664: f64, t6670: f64, t7541: f64, t82312: f64, t870: f64, t87975: f64, t98054: f64) -> (f64, f64, f64) {
    let t105830 = t105776 + t105829;
    let t106348 = t3 * t20217;
    let t106606 = -18.0_f64 * t2522 * t25358 * t28248 - t1877 * t6670 * t21066 - 18.0_f64 * t4314 * t6670 * t105762 + 18.0_f64 * t4314 * t1915 * t20947 + 18.0_f64 * t2522 * t23295 * t105731 - 6.0_f64 * t1877 * t82312 * t20778 + 6.0_f64 * t1877 * t87975 * t5664 + 6.0_f64 * t1877 * t23295 * t105769 + 6.0_f64 * t193 * t20756 * t1914 * t870 + 18.0_f64 * t4314 * t7541 * t5527 + 9.0_f64 * t2522 * t28448 * t1484 - 9.0_f64 * t2522 * t6670 * t105758 - 9.0_f64 * t2522 * t6670 * t105754 + 3.0_f64 * t2522 * t1915 * t20800 - 3.0_f64 * t1877 * t25358 * t5660 - 3.0_f64 * t1877 * t98054 * t1530 + 9.0_f64 * t2522 * t7541 * t5544 + t193 * t202 * t105726 * t870;
    (t105830, t106348, t106606)
}
