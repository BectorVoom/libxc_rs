//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1237/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1237(t101226: f64, t105731: f64, t105754: f64, t105758: f64, t105762: f64, t105769: f64, t108451: f64, t1484: f64, t1530: f64, t1877: f64, t193: f64, t202: f64, t2056: f64, t2057: f64, t20756: f64, t20778: f64, t20800: f64, t20947: f64, t21066: f64, t24344: f64, t2522: f64, t26744: f64, t28248: f64, t29106: f64, t4314: f64, t5527: f64, t5544: f64, t5660: f64, t5664: f64, t7114: f64, t7845: f64, t84766: f64, t870: f64, t93000: f64) -> f64 {
    let t108522 = -6.0_f64 * t1877 * t84766 * t20778 + 6.0_f64 * t1877 * t24344 * t105769 - 9.0_f64 * t2522 * t7114 * t105758 - 9.0_f64 * t2522 * t7114 * t105754 + 18.0_f64 * t4314 * t2057 * t20947 + 9.0_f64 * t2522 * t29106 * t1484 - t1877 * t7114 * t21066 - 3.0_f64 * t1877 * t26744 * t5660 + 6.0_f64 * t193 * t20756 * t2056 * t870 + 18.0_f64 * t4314 * t7845 * t5527 + 9.0_f64 * t2522 * t7845 * t5544 - 18.0_f64 * t4314 * t7114 * t105762 - 18.0_f64 * t2522 * t26744 * t28248 + 18.0_f64 * t2522 * t24344 * t105731 + 6.0_f64 * t1877 * t93000 * t5664 - 3.0_f64 * t1877 * t101226 * t1530 + t193 * t202 * t108451 * t870 + 3.0_f64 * t2522 * t2057 * t20800;
    t108522
}
