//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 854/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk854<F: Float>(t14245: F, t421: F, t5900: F, t1147: F, t794: F, t1193: F, t1354: F, t1798: F, t740: F, t1186: F, t5617: F, t2329: F, t2837: F, t1179: F, t419: F, t5613: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14246 = 0.15917832887339686 * t14245;
    let t14275 = t5900 * t421;
    let t14277 = t1147 * t794;
    let t14279 = t14277 * t1193 * t1354;
    let t14281 = t740 * t1798;
    let t14283 = t14281 * t1193 * t1354;
    let t14284 = 0.0014862827083471494 * t14283;
    let t14290 = t5617 * t1186 * t421;
    let t14291 = 0.01185233419734569 * t14290;
    let t14293 = t2329 * t2837 * t421;
    let t14297 = t1179 * t1798 * t419 * t421;
    let t14298 = 0.01975389032890948 * t14297;
    let t14300 = t5613 * t1186 * t421;
    (t14246, t14275, t14277, t14279, t14281, t14284, t14291, t14293, t14298, t14300)
}
