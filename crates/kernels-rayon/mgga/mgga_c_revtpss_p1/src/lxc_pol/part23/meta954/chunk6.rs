//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3179/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3179(t13052: f64, t24667: f64, t3172: f64, t12916: f64, t24705: f64, t3718: f64, t1222: f64, t1791: f64, t21095: f64, t21177: f64, t21275: f64, t24741: f64, t44624: f64, t5308: f64, t5320: f64, t57464: f64, t57471: f64, t70469: f64, t70685: f64, t70689: f64, t81194: f64) -> f64 {
    let t83485 = t13052 * t3172 * t24667;
    let t83490 = t3718 * t12916 * t24705;
    let t83502 = -0.85748036236139473947e-3_f64 * t21275 * t21095 - 0.85748036236139473947e-3_f64 * t83485 + 0.12862205435420921092e-2_f64 * t44624 * t24741 - 0.42874018118069736972e-3_f64 * t83490 - 0.85748036236139473944e-3_f64 * t70685 + 0.17149607247227894789e-2_f64 * t70689 - t1222 * t5308 * t81194 / 12.0_f64 + t57464 - 0.19055119163586549765e-3_f64 * t57471 - 0.21722835846488666732e-1_f64 * t70469 * t1791 - 0.21722835846488666732e-1_f64 * t21177 * t5320;
    t83502
}
