//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1117/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1117<F: Float>(t237: F, t5845: F, t20638: F, t7308: F, t5906: F, t730: F, t7531: F, t2875: F, t5754: F, t1987: F, t7228: F, t1107: F, t17637: F, t5846: F, t20849: F, t20904: F, t20913: F, t20916: F, t20921: F, t20924: F) -> (F, F, F, F, F, F, F) {
    let t21267 = t237 * t5845;
    let t21270 = 0.30762056574649219974e4 * t21267 * t7308 * t20638;
    let t21273 = 0.14035736694323150897e2 * t730 * t7531 * t5906;
    let t21275 = 0.51947577317044391277e2 * t5754 * t2875;
    let t21277 = 0.30762056574649219973e4 * t1987 * t7228;
    let t21281 = 0.12304822629859687989e5 * t730 * t17637 * t1107 * t5846;
    let t21283 = 0.19751673498613801407e-1 * t237 * t20849;
    let t21284 = t20904 - t21270 + t21273 - t21275 - t20913 + t20916 + t20921 + t20924 - t21277 + t21281 + t21283;
    (t21270, t21273, t21275, t21277, t21281, t21283, t21284)
}
