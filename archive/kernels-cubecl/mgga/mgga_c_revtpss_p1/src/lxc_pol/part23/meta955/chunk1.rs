//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3183/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3183<F: Float>(t21101: F, t5273: F, t1032: F, t1246: F, t24698: F, t1252: F, t12910: F, t20903: F, t24713: F, t3720: F, t5293: F, t5299: F, t5405: F, t56803: F, t56806: F, t6631: F, t6635: F, t69906: F, t70806: F, t70809: F, t70811: F, t70857: F) -> F {
    let t83603 = t5273 * t21101;
    let t83607 = t24698 * t1032 * t1246;
    let t83617 = F::cast_from(0.11433071498151929859e-2_f64) * t70806 + F::cast_from(0.19055119163586549765e-3_f64) * t70809 + F::cast_from(0.17149607247227894789e-2_f64) * t70811 + F::cast_from(0.42874018118069736972e-3_f64) * t69906 * t5299 + F::cast_from(0.12862205435420921092e-2_f64) * t56803 * t6631 - F::cast_from(0.64311027177104605458e-3_f64) * t56806 * t6635 + F::cast_from(0.21722835846488666732e-1_f64) * t83603 * t1252 + F::cast_from(0.21437009059034868486e-3_f64) * t83607 * t1252 - F::cast_from(0.34299214494455789577e-2_f64) * t5293 * t20903 + F::cast_from(0.12862205435420921092e-2_f64) * t12910 * t3720 * t24713 * t5405 - F::cast_from(0.57165357490759649295e-3_f64) * t70857;
    t83617
}
