//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1167/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1167(t1545: f64, t31824: f64, t1416: f64, t1992: f64, t30154: f64, t7586: f64, t1345: f64, t30148: f64, t7842: f64, t34569: f64, t8465: f64, t5281: f64, t7561: f64) -> (f64, f64, f64, f64, f64) {
    let t35987 = t31824 * t1545;
    let t35988 = 0.34299214494455789578e-2_f64 * t35987;
    let t35991 = t30154 * t7586 * t1992 * t1416;
    let t35992 = 0.20965394859736101378e-2_f64 * t35991;
    let t35995 = t30154 * t7842 * t30148 * t1345;
    let t35997 = t34569 * t8465;
    let t35998 = 0.94344276868812456204e-2_f64 * t35997;
    let t35999 = t7561 * t5281;
    (t35988, t35992, t35995, t35998, t35999)
}
