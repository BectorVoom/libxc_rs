//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 955/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk955(t745: f64, t9385: f64, t9368: f64, t2514: f64, t746: f64, t2495: f64, t744: f64, t2576: f64, t2582: f64, t2584: f64, t700: f64) -> (f64, f64, f64, f64, f64) {
    let t9485 = t9385 * t745;
    let t9488 = t9368 * t745;
    let t9501 = t746 * t2514;
    let t9507 = t2514 * t2495;
    let t9508 = t9507 * t744;
    let t9514 = 0.48245938496077605201e2_f64 * t2582 * t2576 * t2584 * t700;
    (t9485, t9488, t9501, t9508, t9514)
}
