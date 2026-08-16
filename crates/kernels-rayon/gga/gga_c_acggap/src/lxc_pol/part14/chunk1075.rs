//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1075/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1075(t1089: f64, t2090: f64, t27338: f64, t598: f64, t30364: f64, t6184: f64, t1988: f64, t9681: f64, t1841: f64, t7685: f64, t1426: f64, t429: f64, t9536: f64) -> (f64, f64, f64, f64, f64) {
    let t38909 = t598 * t1089 * t27338 * t2090;
    let t38912 = t30364 * t6184;
    let t38914 = t1988 * t9681;
    let t38916 = t7685 * t1841;
    let t38920 = t598 * t1426 * t429 * t9536;
    (t38909, t38912, t38914, t38916, t38920)
}
