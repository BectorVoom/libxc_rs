//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 632/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk632(t157: f64, t6068: f64, t175: f64, t398: f64, t1772: f64, t372: f64, t1083: f64, t1795: f64, t322: f64, t1095: f64, t384: f64, t1165: f64, t1879: f64, t407: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6069 = t6068 * t157;
    let t6071 = t398 * t175 * t6069;
    let t6074 = t1772 * t372;
    let t6076 = t398 * t1083 * t6074;
    let t6079 = t1795 * t322;
    let t6081 = t398 * t1095 * t6079;
    let t6082 = t384 * t6081;
    let t6086 = t1165 * t1879 * t407;
    (t6069, t6071, t6074, t6076, t6079, t6081, t6082, t6086)
}
