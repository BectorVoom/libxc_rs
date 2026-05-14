//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1331/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1331<F: Float>(t11641: F, t2782: F, t10: F, t1096: F, t11611: F, t2712: F, t4510: F, t2715: F, t1101: F, t479: F, t23832: F, t23835: F, t23976: F, t23980: F, t23984: F, t23985: F, t27820: F, t27823: F, t27828: F, t27830: F, t27832: F, t27834: F) -> (F,) {
    let t32519 = t11641 * t2782;
    let t32522 = t11611 * t10 * t1096;
    let t32524 = t2712 * t4510;
    let t32526 = t2715 * t4510;
    let t32531 = t11611 * t479 * t1101;
    let t32538 = -32.0 * t27820 + 0.24415263074675393405e-3 * t32519 - 0.36622894612013090108e-3 * t32522 + 12.0 * t32524 - 32.0 * t32526 - 24.0 * t23976 - 8.0 * t27823 - 0.11696447245269292414e1 * t32531 - 0.36622894612013090108e-3 * t27828 + 0.70178683471615754484e1 * t27830 - 0.10389515463408878255e3 * t27832 + 0.97661052298701573622e-3 * t27834 - t23980 + t23984 - t23832 + 0.24415263074675393405e-3 * t23985 + t23835;
    (t32538,)
}
