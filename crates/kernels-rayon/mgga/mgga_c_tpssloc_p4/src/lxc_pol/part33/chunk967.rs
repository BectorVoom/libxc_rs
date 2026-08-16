//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 967/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk967(t10143: f64, t1484: f64, t16625: f64, t193: f64, t202: f64, t20777: f64, t20778: f64, t20800: f64, t20815: f64, t21066: f64, t2522: f64, t4310: f64, t5544: f64, t766: f64, t870: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64) -> f64 {
    let t21073 = 2.0_f64 * t10143 * t193 * t202 * t20778 + t193 * t202 * t21066 * t870 - 9.0_f64 * t1484 * t16625 * t2522 + 3.0_f64 * t193 * t20800 * t766 + 9.0_f64 * t2522 * t4310 * t5544 - t20777 + t20815 - t9820 - t9824 - t9876 - t9884 + t9887 + t9890;
    t21073
}
