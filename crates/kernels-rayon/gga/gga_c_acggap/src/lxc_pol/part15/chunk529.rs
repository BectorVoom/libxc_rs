//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 529/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk529(t1165: f64, t407: f64, t991: f64, t1163: f64, t1171: f64, t3370: f64, t1170: f64, t1162: f64, t3088: f64, t1037: f64, t3073: f64, t322: f64, t944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3427 = t1165 * t991 * t407;
    let t3428 = t1163 * t3427;
    let t3430 = t3370 * t1171;
    let t3431 = t1170 * t3430;
    let t3451 = t3088 * t1162;
    let t3453 = t1165 * t1037 * t407;
    let t3454 = t3451 * t3453;
    let t3456 = t3073 * t1162;
    let t3457 = t944 * t322;
    (t3427, t3428, t3431, t3451, t3453, t3454, t3456, t3457)
}
