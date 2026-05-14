//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 868/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk868<F: Float>(t34760: F, t9221: F, t7457: F, t16503: F, t2281: F, t34962: F, t7467: F, t14237: F, t7482: F, t2402: F, t833: F, t1587: F, t2124: F, t1652: F, t7567: F, t352: F, t8915: F) -> (F, F, F, F, F, F, F) {
    let t40771 = t9221 * t34760;
    let t40772 = t40771 * t7457;
    let t40776 = t16503 * t34962 * t2281 * t7467;
    let t40780 = t16503 * t14237 * t2281 * t7482;
    let t40785 = t2402 * t833;
    let t40788 = t2124 * t1587;
    let t40791 = t7567 * t1652;
    let t40802 = t8915 * t352;
    (t40772, t40776, t40780, t40785, t40788, t40791, t40802)
}
