//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 888/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk888<F: Float>(t11711: F, t1592: F, t3308: F, t7615: F, t2196: F, t10760: F, t7922: F, t6085: F, t7605: F, t6093: F, t11691: F, t11694: F, t11697: F, t11700: F, t11703: F, t11706: F, t11709: F) -> (F, F, F, F) {
    let t11712 = t1592 * t11711;
    let t11714 = t3308 * t7615;
    let t11715 = t2196 * t11714;
    let t11717 = t10760 * t7922;
    let t11718 = t6085 * t11717;
    let t11720 = t10760 * t7605;
    let t11721 = t6093 * t11720;
    let t11723 = 0.54878743191129263322e-1 * t11691 + 0.86682217400542685632e-1 * t11694 + 0.23804984598836975486e-2 * t11697 + 0.71414953796510926457e-2 * t11700 - 0.27439371595564631661e-1 * t11703 + 0.86682217400542685632e-1 * t11706 + 0.13002332610081402845e0 * t11709 + 0.13002332610081402845e0 * t11712 + 0.5200933044032561138e0 * t11715 - 0.21831846657716620896e-2 * t11718 - 0.65495539973149862688e-2 * t11721;
    (t11714, t11717, t11720, t11723)
}
