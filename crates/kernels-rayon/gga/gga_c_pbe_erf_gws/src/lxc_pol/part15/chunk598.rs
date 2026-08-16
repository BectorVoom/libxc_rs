//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 598/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk598(t2722: f64, t650: f64, t186: f64, t211: f64, t1033: f64, t663: f64, t209: f64, t617: f64, t184: f64, t1024: f64, t1730: f64, t1: f64, t331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2723 = t650 * t2722;
    let t2724 = t186 * t2723;
    let t2726 = 2.0_f64 / 15.0_f64 * t211 * t2724;
    let t2728 = 2.0_f64 / 15.0_f64 * t1033 * t663;
    let t2729 = t617 * t209;
    let t2730 = t2729 * t184;
    let t2732 = 4.0_f64 / 15.0_f64 * t2730 * t1024;
    let t2734 = 4.0_f64 / 15.0_f64 * t1730 * t1024;
    let t2735 = t1 * t331;
    (t2723, t2724, t2726, t2728, t2729, t2730, t2732, t2734, t2735)
}
