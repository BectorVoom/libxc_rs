//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1104/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1104(t101: f64, t8449: f64, t203: f64, t8958: f64, t103: f64, t567: f64, t1303: f64, t147: f64, t19: f64, t3156: f64, t4864: f64, t1908: f64, t3940: f64) -> (f64, f64, f64, f64, f64) {
    let t25042 = t8449 * t101;
    let t25045 = t8958 * t203;
    let t25047 = t25045 * t103 * t567;
    let t25054 = t3156 * t1303 * t19 * t147;
    let t25076 = t4864 * t19;
    let t25110 = t3940 * t1908;
    (t25042, t25047, t25054, t25076, t25110)
}
