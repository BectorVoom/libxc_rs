//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1758/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1758(t22724: f64, t22727: f64, t22894: f64, t80670: f64, t154: f64, t9533: f64, t131: f64, t3748: f64, t2009: f64, t9537: f64, t22642: f64, t22690: f64, t22881: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81076 = t22724 * t22727;
    let t81080 = t80670 * t22894;
    let t81142 = t9533 * t154;
    let t81144 = t81142 * t3748 * t131;
    let t81146 = t81144 * t9537 * t2009;
    let t81149 = t22642 * t22690 * t22881;
    (t81076, t81080, t81142, t81144, t81146, t81149)
}
