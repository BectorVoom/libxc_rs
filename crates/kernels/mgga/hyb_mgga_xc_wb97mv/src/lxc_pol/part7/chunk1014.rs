//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1014/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1014<F: Float>(t1142: F, t536: F, t1157: F, t2952: F, t537: F, t1111: F, t1514: F, t1106: F, t2831: F, t522: F, t7817: F, t535: F, t2822: F, t1126: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10146 = t536 * t1142;
    let t10147 = t1157 * t10146;
    let t10150 = t2952 * t537;
    let t10151 = t1514 * t1111;
    let t10152 = t1106 * t10151;
    let t10155 = t2831 * t522;
    let t10156 = t1157 * t10155;
    let t10161 = t7817 * t522;
    let t10162 = t535 * t10161;
    let t10165 = t2822 * t522;
    let t10166 = t1126 * t10165;
    (t10146, t10147, t10150, t10152, t10155, t10156, t10161, t10162, t10165, t10166)
}
