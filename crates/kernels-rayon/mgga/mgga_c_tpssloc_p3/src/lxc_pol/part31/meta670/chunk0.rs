//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1989/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1989(t101832: f64, t870: f64, t193: f64, t7859: f64, t16557: f64, t1877: f64, t2057: f64, t24191: f64, t24339: f64, t25: f64, t25024: f64, t2522: f64, t25375: f64, t25377: f64, t25385: f64, t26563: f64, t26744: f64, t28256: f64, t28459: f64, t29106: f64, t4314: f64, t606: f64, t7110: f64, t7114: f64, t7845: f64, t97950: f64, t97953: f64, t97985: f64, t98015: f64, t98034: f64, t98075: f64) -> (f64, f64, f64) {
    let t101833 = t101832 * t870;
    let t101840 = t193 * t7859;
    let t101843 = 6.0_f64 * t26563 * t97950 - 3.0_f64 * t24191 * t97953 - 3.0_f64 * t24191 * t98015 + t1877 * t29106 * t606 / 2.0_f64 + t1877 * t2057 * t16557 / 2.0_f64 + 3.0_f64 * t2522 * t7845 * t25385 - t1877 * t24339 * t28459 + 3.0_f64 / 2.0_f64 * t2522 * t7110 * t28256 - 3.0_f64 / 2.0_f64 * t24191 * t98034 + 3.0_f64 * t2522 * t7845 * t25024 - t1877 * t26744 * t25377 - t1877 * t7114 * t98075 / 2.0_f64 + t1877 * t101833 * t25 / 2.0_f64 + 3.0_f64 * t4314 * t2057 * t97985 + 2.0_f64 * t101840 * t25375;
    (t101833, t101840, t101843)
}
