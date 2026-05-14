//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1268/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1268<F: Float>(t11096: F, t2325: F, t4242: F, t6968: F, t11097: F, t11126: F, t11131: F, t11132: F, t11135: F, t11138: F, t11139: F, t22394: F, t22569: F, t22571: F, t2301: F, t2302: F, t2317: F, t2323: F, t26853: F, t26856: F, t26929: F, t3388: F, t3407: F, t3439: F, t4229: F, t4243: F, t6929: F, t6966: F, t6977: F, t6982: F, t846: F, t9000: F, t9061: F, t9084: F, t9166: F, t9169: F, t9173: F, t9177: F) -> (F,) {
    let t31066 = t11096 * t2325;
    let t31073 = t4242 * t6968;
    let t31103 = -0.23392894490538584828e1 * t6929 * t11126 - 0.23392894490538584828e1 * t2301 * t11097 * t846 - 0.11696447245269292414e1 * t2301 * t4243 * t2317 - 0.10389515463408878255e3 * t6982 * t11131 * t2302 + 0.34631718211362927518e2 * t6977 * t11132 + 0.34631718211362927518e2 * t2323 * t31066 * t846 + 0.17315859105681463759e2 * t2323 * t11131 * t2317 + 0.10254018858216406658e4 * t6966 * t31073 * t2302 + 0.69263436422725855036e2 * t6977 * t11135 + 0.34631718211362927518e2 * t2323 * t3439 * t9000 + 0.20508037716432813316e4 * t22394 * t11139 + 0.10254018858216406658e4 * t6966 * t11138 * t2317 + 0.91082604192152556044e5 * t22569 * t4229 * t22571 * t2302 - 8.0 * t26853 * t3388 - 8.0 * t9084 * t9166 - 4.0 * t9084 * t9169 - 0.38596750796862084161e3 * t26856 * t9173 + 0.12865583598954028054e3 * t26929 * t3407 + 0.12865583598954028054e3 * t9061 * t9177;
    (t31103,)
}
