//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2270/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2270(t2363: f64, t3941: f64, t7467: f64, t12724: f64, t12728: f64, t16503: f64, t2165: f64, t2167: f64, t2364: f64, t24552: f64, t27858: f64, t27863: f64, t4028: f64, t4072: f64, t650: f64, t652: f64, t7408: f64, t7989: f64, t86673: f64, t86676: f64, t86679: f64, t86682: f64, t86684: f64, t86688: f64, t86693: f64, t86698: f64, t86700: f64, t86702: f64, t90020: f64, t9348: f64) -> (f64, f64) {
    let t91802 = 27.0_f64 * t3941 * t7467 * t2363;
    let t94223 = -4.0_f64 * t4072 * t652 * t7408 - t12724 * t2165 - 2.0_f64 * t12728 * t2165 + t16503 * t2167 - 2.0_f64 * t2364 * t27863 - 2.0_f64 * t24552 * t4028 - 2.0_f64 * t27858 * t650 - 2.0_f64 * t7989 * t9348 + t86673 + t86676 + t86679 + t86682 - t86684 - t86688 + t86693 - t86698 - t86700 - t86702 + t90020;
    (t91802, t94223)
}
