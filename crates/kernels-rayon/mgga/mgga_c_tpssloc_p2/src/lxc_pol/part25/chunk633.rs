//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 633/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk633(t1215: f64, t1932: f64, t475: f64, t671: f64, t88: f64, t193: f64, t531: f64, t533: f64, t131: f64, t3732: f64, t205: f64, t242: f64, t3788: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5079 = t1932 * t1215 * t475;
    let t5113 = t88 * t671;
    let t5126 = t193 * t531;
    let t5160 = t193 * t533;
    let t5194 = t3732 * t131;
    let t5195 = t205 * t5194;
    let t5245 = t3788 * t242;
    (t5079, t5113, t5126, t5160, t5195, t5245)
}
