//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1270/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1270(t16409: f64, t342: f64, t1071: f64, t3316: f64, t4980: f64, t989: f64, t4995: f64, t12166: f64, t378: f64, t11631: f64, t12050: f64, t12077: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16410 = t342 * t16409;
    let t16505 = t3316 * t1071;
    let t16506 = t342 * t16505;
    let t16520 = t989 * t4980;
    let t16523 = t989 * t4995;
    let t16551 = t12166 * t378;
    let t16552 = t342 * t16551;
    let t16553 = t12050 * t11631;
    let t16558 = t12077 * t378;
    (t16410, t16505, t16506, t16520, t16523, t16551, t16552, t16553, t16558)
}
