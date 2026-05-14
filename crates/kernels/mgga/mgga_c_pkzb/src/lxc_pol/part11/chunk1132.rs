//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1132/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1132<F: Float>(t3551: F, t7489: F, t2746: F, t9390: F, t21221: F, t3554: F, t10901: F, t17329: F, t20893: F, t3525: F, t17351: F, t17728: F, t20705: F, t21191: F, t228: F, t25633: F, t25636: F, t30284: F, t30287: F) -> (F, F, F, F, F, F) {
    let t30620 = 3.0 * t7489 * t3551;
    let t30622 = 3.0 * t2746 * t9390;
    let t30624 = 0.48245938496077605201e2 * t21221 * t3554;
    let t30626 = 0.96491876992155210402e2 * t17329 * t10901;
    let t30628 = 6.0 * t20893 * t3525;
    let t30637 = 0.621814e-1 * (t17728 - 0.55403703703703703703e-1 * t17351 - 0.16621111111111111111e0 * t20705 + t21191 + 0.71233333333333333332e-1 * t25633 - 0.53424999999999999999e-1 * t25636 - 0.17808333333333333333e-1 * t30284 + 0.53425e-1 * t30287) * t228;
    (t30620, t30622, t30624, t30626, t30628, t30637)
}
