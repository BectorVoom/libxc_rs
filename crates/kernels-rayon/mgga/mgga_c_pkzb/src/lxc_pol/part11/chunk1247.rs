//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1247/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1247(t3551: f64, t7489: f64, t2746: f64, t9390: f64, t21221: f64, t3554: f64, t10901: f64, t17329: f64, t20893: f64, t3525: f64, t17351: f64, t17728: f64, t20705: f64, t21191: f64, t228: f64, t25633: f64, t25636: f64, t30284: f64, t30287: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30620 = 3.0_f64 * t7489 * t3551;
    let t30622 = 3.0_f64 * t2746 * t9390;
    let t30624 = 0.48245938496077605201e2_f64 * t21221 * t3554;
    let t30626 = 0.96491876992155210402e2_f64 * t17329 * t10901;
    let t30628 = 6.0_f64 * t20893 * t3525;
    let t30637 = 0.621814e-1_f64 * (t17728 - 0.55403703703703703703e-1_f64 * t17351 - 0.16621111111111111111e0_f64 * t20705 + t21191 + 0.71233333333333333332e-1_f64 * t25633 - 0.53424999999999999999e-1_f64 * t25636 - 0.17808333333333333333e-1_f64 * t30284 + 0.53425e-1_f64 * t30287) * t228;
    (t30620, t30622, t30624, t30626, t30628, t30637)
}
