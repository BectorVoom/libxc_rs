//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 834/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk834<F: Float>(t40756: F, t903: F, t1679: F, t7197: F, t7200: F, t38530: F, t7484: F, t7450: F, t34760: F, t9221: F, t7457: F, t16503: F, t2281: F, t34962: F, t7467: F, t14237: F, t7482: F) -> (F, F, F, F, F, F, F) {
    let t40757 = t903 * t40756;
    let t40759 = t1679 * t7197;
    let t40760 = t40759 * t7200;
    let t40762 = t38530 * t7484;
    let t40764 = t38530 * t7450;
    let t40771 = t9221 * t34760;
    let t40772 = t40771 * t7457;
    let t40776 = t16503 * t34962 * t2281 * t7467;
    let t40780 = t16503 * t14237 * t2281 * t7482;
    (t40757, t40760, t40762, t40764, t40772, t40776, t40780)
}
