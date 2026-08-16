//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 687/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk687(t3242: f64, t3584: f64, t1215: f64, t1932: f64, t475: f64, t671: f64, t88: f64, t193: f64, t531: f64, t533: f64, t131: f64, t3732: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4987 = t3584 * t3242;
    let t5079 = t1932 * t1215 * t475;
    let t5113 = t88 * t671;
    let t5126 = t193 * t531;
    let t5160 = t193 * t533;
    let t5194 = t3732 * t131;
    (t4987, t5079, t5113, t5126, t5160, t5194)
}
