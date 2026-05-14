//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1142/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1142<F: Float>(t1026: F, t1035: F, t2659: F, t2664: F, t2667: F, t259: F, t457: F, t463: F, t491: F, t2685: F, t7540: F, t2697: F, t7525: F, t2694: F, t2704: F, t2626: F, t2647: F) -> (F, F, F, F, F, F, F) {
    let t23552 = 0.34367190188705947437e1 * t1026 * t2664 * t2659 * t2667 * t1035;
    let t23556 = 24.0 * t457 * t463 * t259 * t491;
    let t23557 = t7540 * t2685;
    let t23559 = t2697 * t7525;
    let t23561 = t7540 * t2694;
    let t23563 = t7540 * t2704;
    let t23567 = 0.86748650402413918736e-1 * t2626 * t2647 * t2704;
    (t23552, t23556, t23557, t23559, t23561, t23563, t23567)
}
