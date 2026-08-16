//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1173/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1173(t1866: f64, t361: f64, t7436: f64, t142: f64, t6304: f64, t1998: f64, t5971: f64, t1426: f64, t1894: f64, t2085: f64, t598: f64, t1967: f64, t9549: f64) -> (f64, f64, f64, f64, f64) {
    let t40313 = t7436 * t361 * t1866;
    let t40316 = t7436 * t142 * t6304;
    let t40318 = t1998 * t5971;
    let t40322 = t598 * t1426 * t1894 * t2085;
    let t40324 = t1967 * t9549;
    (t40313, t40316, t40318, t40322, t40324)
}
