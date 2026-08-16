//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1336/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1336(t3531: f64, t5202: f64, t300: f64, t5155: f64, t1198: f64, t3539: f64, t5192: f64, t12571: f64, t1765: f64, t16710: f64, t16712: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12382: f64, t16706: f64, t16708: f64, t16717: f64, t16722: f64, t16727: f64, t16731: f64, t16735: f64, t16740: f64, t16744: f64, t16748: f64) -> (f64, f64, f64, f64, f64) {
    let t16783 = 0.11696447245269292414e1_f64 * t3531 * t5202;
    let t16784 = t300 * t5155;
    let t16786 = 0.11696447245269292414e1_f64 * t16784 * t1198;
    let t16788 = 0.5848223622634646207e0_f64 * t5192 * t3539;
    let t16790 = 0.5848223622634646207e0_f64 * t12571 * t1765;
    let t16797 = 0.23744444444444444444e-1_f64 * t16710;
    let t16798 = 0.11872222222222222222e-1_f64 * t16712;
    let t16807 = -t12382 + 0.15829629629629629629e-1_f64 * t12297 + 0.39574074074074074073e-2_f64 * t12299 - 0.11872222222222222222e-1_f64 * t12301 - 0.5936111111111111111e-2_f64 * t12303 + 0.79148148148148148146e-2_f64 * t16706 + 0.79148148148148148146e-2_f64 * t16708 - t16797 - t16798 + 0.19787037037037037037e-1_f64 * t16717 - 0.71233333333333333332e-1_f64 * t16722 - 0.23744444444444444444e-1_f64 * t16727 - 0.11872222222222222222e-1_f64 * t16731 + 0.10685e0_f64 * t16735 + 0.71233333333333333332e-1_f64 * t16740 + 0.35616666666666666666e-1_f64 * t16744 + 0.17808333333333333333e-1_f64 * t16748;
    (t16783, t16786, t16788, t16790, t16807)
}
