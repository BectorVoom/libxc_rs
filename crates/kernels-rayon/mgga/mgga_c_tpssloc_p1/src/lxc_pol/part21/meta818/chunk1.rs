//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2882/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2882(t136: f64, t2826: f64, t59676: f64, t59661: f64, t59663: f64, t59665: f64, t59670: f64, t59674: f64, t59678: f64, t60186: f64, t60189: f64, t60192: f64, t60194: f64, t60197: f64, t60200: f64, t60202: f64, t60204: f64) -> (f64, f64) {
    let t60207 = t136 * t2826 * t59676;
    let t60214 = 0.16504875e0_f64 * t60186 + 0.198684e1_f64 * t60189 + 0.72462e1_f64 * t59661 + 0.66228e0_f64 * t60192 - 0.44152e0_f64 * t60194 - 0.49671e0_f64 * t60197 + 0.33114e0_f64 * t60200 - 0.22076e0_f64 * t60202 - 0.30661111111111111112e-1_f64 * t60204 - 0.5519e-1_f64 * t60207 - 0.40256666666666666667e0_f64 * t59663 + 0.13418888888888888889e0_f64 * t59665 - 0.40256666666666666666e0_f64 * t59670 - 0.20128333333333333333e0_f64 * t59674 - 0.40256666666666666666e0_f64 * t59678;
    (t60207, t60214)
}
