//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1247/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1247<F: Float>(t3551: F, t7489: F, t2746: F, t9390: F, t21221: F, t3554: F, t10901: F, t17329: F, t20893: F, t3525: F, t17351: F, t17728: F, t20705: F, t21191: F, t228: F, t25633: F, t25636: F, t30284: F, t30287: F) -> (F, F, F, F, F, F) {
    let t30620 = F::cast_from(3.0_f64) * t7489 * t3551;
    let t30622 = F::cast_from(3.0_f64) * t2746 * t9390;
    let t30624 = F::cast_from(0.48245938496077605201e2_f64) * t21221 * t3554;
    let t30626 = F::cast_from(0.96491876992155210402e2_f64) * t17329 * t10901;
    let t30628 = F::cast_from(6.0_f64) * t20893 * t3525;
    let t30637 = F::cast_from(0.621814e-1_f64) * (t17728 - F::cast_from(0.55403703703703703703e-1_f64) * t17351 - F::cast_from(0.16621111111111111111e0_f64) * t20705 + t21191 + F::cast_from(0.71233333333333333332e-1_f64) * t25633 - F::cast_from(0.53424999999999999999e-1_f64) * t25636 - F::cast_from(0.17808333333333333333e-1_f64) * t30284 + F::cast_from(0.53425e-1_f64) * t30287) * t228;
    (t30620, t30622, t30624, t30626, t30628, t30637)
}
