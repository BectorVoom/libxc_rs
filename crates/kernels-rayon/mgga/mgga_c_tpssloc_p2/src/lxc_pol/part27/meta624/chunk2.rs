//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2106/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2106(t109: f64, t86603: f64, t1401: f64, t55571: f64, t7769: f64, t20173: f64, t26542: f64, t26545: f64, t12524: f64, t1458: f64, t22479: f64, t3941: f64, t4072: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t86604 = piecewise3(t110, 0.0_f64, t86603);
    let t86606 = 0.135e2_f64 * t1401 * t86604;
    let t86610 = 27.0_f64 * t55571 * t7769;
    let t86612 = 54.0_f64 * t20173 * t26542;
    let t86614 = 54.0_f64 * t20173 * t26545;
    let t86616 = 54.0_f64 * t12524 * t26545;
    let t86619 = 27.0_f64 * t3941 * t22479 * t1458;
    let t86622 = 54.0_f64 * t3941 * t6534 * t4072;
    (t86604, t86606, t86610, t86612, t86614, t86616, t86619, t86622)
}
