//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1068/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1068<F: Float>(t2325: F, t4242: F, t846: F, t3435: F, t3439: F, t4229: F, t6968: F, t4203: F, t827: F, t1353: F, t3402: F, t4219: F, t4216: F, t2286: F, t4215: F, t3406: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11131 = t4242 * t2325;
    let t11132 = t11131 * t846;
    let t11135 = t3439 * t3435;
    let t11138 = t4229 * t6968;
    let t11139 = t11138 * t846;
    let t11146 = t4203 * t827;
    let t11149 = t1353 * t3402;
    let t11152 = t4219 * t827;
    let t11155 = t4216 * t827;
    let t11158 = t4215 * t2286;
    let t11159 = t11158 * t827;
    let t11162 = t3406 * t3402;
    (t11131, t11132, t11135, t11138, t11139, t11146, t11149, t11152, t11155, t11158, t11159, t11162)
}
