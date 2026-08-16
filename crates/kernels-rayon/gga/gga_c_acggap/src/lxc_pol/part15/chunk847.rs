//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 847/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk847(t150: f64, t187: f64, t9971: f64, t1914: f64, t633: f64, t8004: f64, t1814: f64, t7890: f64, t944: f64, t2385: f64, t556: f64, t2147: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9973 = t9971 * t150 * t187;
    let t9976 = t633 * t1914;
    let t9977 = t8004 * t9976;
    let t9980 = t633 * t1814;
    let t9982 = t7890 * t9980 * t944;
    let t9985 = t2385 * t556;
    let t9986 = t2147 * t9985;
    (t9973, t9976, t9977, t9980, t9982, t9985, t9986)
}
