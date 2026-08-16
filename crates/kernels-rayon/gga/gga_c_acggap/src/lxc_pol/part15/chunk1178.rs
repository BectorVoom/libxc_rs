//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1178/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1178(t1998: f64, t6125: f64, t30811: f64, t6090: f64, t30543: f64, t9670: f64, t1165: f64, t39794: f64, t604: f64, t7413: f64, t1181: f64, t30856: f64, t40215: f64, t599: f64) -> (f64, f64, f64, f64, f64) {
    let t40387 = t1998 * t6125;
    let t40390 = t30811 * t6090;
    let t40398 = t30543 * t9670;
    let t40403 = t7413 * t1165 * t604 * t39794;
    let t40408 = t30856 * t1181 * t599 * t40215;
    (t40387, t40390, t40398, t40403, t40408)
}
