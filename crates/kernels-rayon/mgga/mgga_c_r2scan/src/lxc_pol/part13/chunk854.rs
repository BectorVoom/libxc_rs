//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 854/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk854(t229: f64, t7007: f64, t1721: f64, t898: f64, t5393: f64, t2483: f64, t5: f64, t736: f64, t41: f64, t5366: f64, t5373: f64, t5378: f64, t5384: f64, t5392: f64, t5401: f64, t5405: f64, t5409: f64) -> f64 {
    let t7733 = t7007 * t229;
    let t7737 = t898 * t1721;
    let t7739 = 48.0_f64 * t5393;
    let t7741 = t2483 * t5;
    let t7743 = 0.10843581300301739842e-1_f64 * t7741 * t736;
    let t7744 = -t41 * t7733 + t5366 + 0.3429168e0_f64 * t5373 + 0.16008171603946666666e-1_f64 * t5378 + 0.65061487801810439052e-1_f64 * t7737 + t5384 - t5392 - t7739 - t5401 - t5405 + 0.84681398666666666666e-3_f64 * t5409 - t7743;
    t7744
}
