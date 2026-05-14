//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1120/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1120<F: Float>(t11707: F, t11764: F, t11825: F, t11883: F, t11942: F, t12005: F, t12057: F, t12101: F, t496: F, t11717: F, t1291: F, t1142: F, t1801: F, t1122: F, t198: F, t11703: F) -> (F, F, F, F, F, F) {
    let t12104 = t11707 + t11764 + t11825 + t11883 + t11942 + t12005 + t12057 + t12101;
    let t12105 = t496 * t12104;
    let t13473 = t11717 * t1291;
    let t13638 = t1801 * t1142;
    let t13872 = t1122 * t198;
    let t14407 = t11703 * t1291;
    (t12104, t12105, t13473, t13638, t13872, t14407)
}
