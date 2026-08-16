//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3172/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3172(t21213: f64, t5357: f64, t17401: f64, t21166: f64, t21259: f64, t57126: f64, t70378: f64, t70382: f64, t70394: f64, t70403: f64, t70405: f64, t70411: f64, t70427: f64, t70432: f64) -> f64 {
    let t83316 = t21213 * t5357;
    let t83322 = 0.42874018118069736972e-3_f64 * t70378 + 0.95275595817932748825e-3_f64 * t70382 + 0.28582678745379824648e-3_f64 * t70394 - t57126 + 0.30488190661738479624e-2_f64 * t70403 + 0.19055119163586549765e-3_f64 * t70405 + 0.17149607247227894789e-2_f64 * t70411 - 0.57165357490759649295e-3_f64 * t70427 - 0.28582678745379824648e-3_f64 * t70432 - 11.0_f64 / 324.0_f64 * t83316 - 0.12862205435420921092e-2_f64 * t17401 * t21166 - 0.12862205435420921092e-2_f64 * t17401 * t21259;
    t83322
}
