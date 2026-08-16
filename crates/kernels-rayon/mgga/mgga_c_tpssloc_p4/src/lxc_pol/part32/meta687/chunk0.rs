//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2129/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2129(t19451: f64, t6534: f64, t1458: f64, t4025: f64, t1873: f64, t55943: f64, t19456: f64, t7467: f64, t26135: f64, t4028: f64, t5493: f64, t649: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96681 = 2.0_f64 * t19451 * t6534;
    let t96683 = t4025 * t1458;
    let t96685 = 4.0_f64 * t96683 * t1873;
    let t96704 = 2.0_f64 * t55943 * t1873;
    let t96706 = 4.0_f64 * t19456 * t7467;
    let t96708 = 4.0_f64 * t4028 * t26135;
    let t96709 = t649 * t5493;
    (t96681, t96683, t96685, t96704, t96706, t96708, t96709)
}
