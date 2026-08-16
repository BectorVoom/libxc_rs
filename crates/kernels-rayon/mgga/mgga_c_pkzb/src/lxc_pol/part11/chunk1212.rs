//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1212/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1212(t2860: f64, t9359: f64, t10979: f64, t2029: f64, t10937: f64, t2887: f64, t68: f64, t10985: f64, t2099: f64, t5954: f64, t10942: f64, t17938: f64) -> (f64, f64, f64, f64, f64) {
    let t29753 = 0.10389515463408878255e3_f64 * t2860 * t9359;
    let t29754 = t10979 * t2029;
    let t29762 = t2887 * t68 * t10937;
    let t29766 = t5954 * t2099 * t10985;
    let t29775 = t10942 * t17938;
    (t29753, t29754, t29762, t29766, t29775)
}
