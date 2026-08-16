//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 684/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk684(t143: f64, t4597: f64, t3845: f64, t429: f64, t686: f64, t5814: f64, t79: f64, t435: f64, t690: f64, t41: f64, t5821: f64, t698: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11495 = t143 * t4597;
    let t11524 = 0.27323333333333333333e-1_f64 * t429 * t3845 * t686;
    let t11525 = t5814 * t79;
    let t11528 = 0.77488888888888888888e-2_f64 * t435 * t11525 * t690;
    let t11529 = t5821 * t41;
    let t11530 = t11529 * t698;
    (t11495, t11524, t11525, t11528, t11529, t11530)
}
