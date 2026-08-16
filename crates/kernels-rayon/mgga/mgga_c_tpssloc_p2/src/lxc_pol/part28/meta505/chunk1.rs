//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1747/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1747(t27005: f64, t27065: f64, t27127: f64, t27141: f64, t533: f64, t1390: f64, t671: f64, t7890: f64, t2075: f64, t4072: f64, t2039: f64, t5107: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27143 = t27005 + t27065 + t27127 + t27141;
    let t27144 = t533 * t27143;
    let t27145 = t27144 * t1390;
    let t27147 = t7890 * t671;
    let t27150 = t2075 * t4072;
    let t27163 = t5107 * t2039;
    (t27143, t27144, t27145, t27147, t27150, t27163)
}
