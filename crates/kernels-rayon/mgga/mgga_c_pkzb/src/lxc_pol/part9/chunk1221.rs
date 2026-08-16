//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1221/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1221(t237: f64, t5845: f64, t20638: f64, t7308: f64, t5906: f64, t730: f64, t7531: f64, t2875: f64, t5754: f64, t1987: f64, t7228: f64, t1107: f64, t17637: f64, t5846: f64) -> (f64, f64, f64, f64, f64) {
    let t21267 = t237 * t5845;
    let t21270 = 0.30762056574649219974e4_f64 * t21267 * t7308 * t20638;
    let t21273 = 0.14035736694323150897e2_f64 * t730 * t7531 * t5906;
    let t21275 = 0.51947577317044391277e2_f64 * t5754 * t2875;
    let t21277 = 0.30762056574649219973e4_f64 * t1987 * t7228;
    let t21281 = 0.12304822629859687989e5_f64 * t730 * t17637 * t1107 * t5846;
    (t21270, t21273, t21275, t21277, t21281)
}
