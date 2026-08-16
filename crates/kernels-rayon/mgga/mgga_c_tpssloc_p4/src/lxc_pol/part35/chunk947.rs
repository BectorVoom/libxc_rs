//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 947/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk947(t1268: f64, t1458: f64, t19451: f64, t20293: f64, t20296: f64, t20347: f64, t4028: f64, t5493: f64, t7676: f64, t19542: f64, t19576: f64, t1799: f64, t6330: f64) -> (f64, f64, f64, f64) {
    let t20350 = 2.0_f64 * t1268 * t20347 + 6.0_f64 * t1458 * t19451 + 6.0_f64 * t4028 * t5493 + 6.0_f64 * t5493 * t7676 + t20293 + 6.0_f64 * t20296;
    let t20354 = 0.54934341918019635162e-3_f64 * t19542;
    let t20355 = 3.0_f64 * t19576;
    let t20356 = t6330 * t1799;
    (t20350, t20354, t20355, t20356)
}
