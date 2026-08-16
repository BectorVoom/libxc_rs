//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3183/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3183(t21101: f64, t5273: f64, t1032: f64, t1246: f64, t24698: f64, t1252: f64, t12910: f64, t20903: f64, t24713: f64, t3720: f64, t5293: f64, t5299: f64, t5405: f64, t56803: f64, t56806: f64, t6631: f64, t6635: f64, t69906: f64, t70806: f64, t70809: f64, t70811: f64, t70857: f64) -> f64 {
    let t83603 = t5273 * t21101;
    let t83607 = t24698 * t1032 * t1246;
    let t83617 = 0.11433071498151929859e-2_f64 * t70806 + 0.19055119163586549765e-3_f64 * t70809 + 0.17149607247227894789e-2_f64 * t70811 + 0.42874018118069736972e-3_f64 * t69906 * t5299 + 0.12862205435420921092e-2_f64 * t56803 * t6631 - 0.64311027177104605458e-3_f64 * t56806 * t6635 + 0.21722835846488666732e-1_f64 * t83603 * t1252 + 0.21437009059034868486e-3_f64 * t83607 * t1252 - 0.34299214494455789577e-2_f64 * t5293 * t20903 + 0.12862205435420921092e-2_f64 * t12910 * t3720 * t24713 * t5405 - 0.57165357490759649295e-3_f64 * t70857;
    t83617
}
