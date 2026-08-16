//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 998/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk998(t11717: f64, t6085: f64, t10760: f64, t7605: f64, t6093: f64, t11691: f64, t11694: f64, t11697: f64, t11700: f64, t11703: f64, t11706: f64, t11709: f64, t11712: f64, t11715: f64) -> (f64, f64) {
    let t11718 = t6085 * t11717;
    let t11720 = t10760 * t7605;
    let t11721 = t6093 * t11720;
    let t11723 = 0.54878743191129263322e-1_f64 * t11691 + 0.86682217400542685632e-1_f64 * t11694 + 0.23804984598836975486e-2_f64 * t11697 + 0.71414953796510926457e-2_f64 * t11700 - 0.27439371595564631661e-1_f64 * t11703 + 0.86682217400542685632e-1_f64 * t11706 + 0.13002332610081402845e0_f64 * t11709 + 0.13002332610081402845e0_f64 * t11712 + 0.5200933044032561138e0_f64 * t11715 - 0.21831846657716620896e-2_f64 * t11718 - 0.65495539973149862688e-2_f64 * t11721;
    (t11720, t11723)
}
