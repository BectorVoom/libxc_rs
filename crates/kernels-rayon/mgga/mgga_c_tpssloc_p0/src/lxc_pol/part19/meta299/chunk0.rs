//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1083/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1083(t1090: f64, t7319: f64, t11545: f64, t60: f64, t461: f64, t11588: f64, t134: f64, t3439: f64, t3507: f64, t475: f64, t6739: f64, t11147: f64, t11778: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15288 = t7319 * t1090;
    let t15394 = t60 * t11545;
    let t15395 = t15394 * t461;
    let t15402 = t11588 * t461;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    let t15429 = t6739 * t3507 * t475;
    let t15453 = t11778 * t11147;
    (t15288, t15394, t15395, t15402, t15418, t15419, t15429, t15453)
}
