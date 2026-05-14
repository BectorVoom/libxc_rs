//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1362/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1362<F: Float>(t11826: F, t2895: F, t4558: F, t7823: F, t1128: F, t2869: F, t4529: F, t2873: F, t4550: F, t11821: F, t1127: F, t1132: F, t1158: F, t1161: F, t11794: F, t12038: F, t2915: F, t2946: F, t2953: F, t33504: F, t33507: F, t33510: F, t4541: F, t4613: F, t4616: F, t7908: F, t7913: F, t7927: F, t8025: F, t8081: F, t8094: F, t9974: F) -> (F, F, F, F, F, F, F) {
    let t33516 = t2895 * t11826;
    let t33519 = t4558 * t7823;
    let t33540 = t1128 * t4529 * t2869;
    let t33544 = t1128 * t4529 * t2873;
    let t33548 = t1128 * t4550 * t2873;
    let t33551 = t2895 * t11821;
    let t33560 = 0.768e-3 * t1161 * t33516 - 0.768e-3 * t1158 * t33519 - 0.768e-3 * t2946 * t33504 + 0.58666666666666666666e-1 * t1127 * t33507 - 0.58666666666666666666e-1 * t1132 * t33510 + 0.36e0 * t7927 * t1128 * t4541 * t2869 - 0.54e0 * t8025 * t1128 * t4541 * t2873 - 0.384e-2 * t2953 * t2895 * t11794 + 0.756e0 * t8081 * t33540 - 0.1008e1 * t7908 * t33544 - 0.108e0 * t2915 * t33548 - 0.768e-3 * t1158 * t33551 + 0.64e-1 * t8094 * t4613 + 0.768e-6 * t7913 * t4616 - 800.0 / 27.0 * t9974 * t12038;
    (t33516, t33519, t33540, t33544, t33548, t33551, t33560)
}
