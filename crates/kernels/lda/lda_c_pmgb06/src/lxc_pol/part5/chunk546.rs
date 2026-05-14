//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 546/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk546<F: Float>(t170: F, t3457: F, t117: F, t123: F, t550: F, t740: F, t1135: F, t118: F, t103: F, t37: F, t28: F, t39: F, t247: F, t61: F, t939: F, t64: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3458 = t170 * t3457;
    let t3474 = t123 * t740 * t550 * t117;
    let t3481 = 0.1890324433388467 * t1135 * t118;
    let t3500 = 1.0 / t37 / t103 / 4.0;
    let t3501 = param_hyb_omega_0 * t3500;
    let t3502 = t39 * t28;
    let t3505 = 1.9486833333333333 * t3501 * t3502 * t247;
    let t3509 = t61 * t939;
    let t3510 = t64 * t28;
    (t3458, t3474, t3481, t3500, t3501, t3502, t3505, t3509, t3510)
}
