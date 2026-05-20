//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3179/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3179<F: Float>(t13052: F, t24667: F, t3172: F, t12916: F, t24705: F, t3718: F, t1222: F, t1791: F, t21095: F, t21177: F, t21275: F, t24741: F, t44624: F, t5308: F, t5320: F, t57464: F, t57471: F, t70469: F, t70685: F, t70689: F, t81194: F) -> F {
    let t83485 = t13052 * t3172 * t24667;
    let t83490 = t3718 * t12916 * t24705;
    let t83502 = -F::cast_from(0.85748036236139473947e-3_f64) * t21275 * t21095 - F::cast_from(0.85748036236139473947e-3_f64) * t83485 + F::cast_from(0.12862205435420921092e-2_f64) * t44624 * t24741 - F::cast_from(0.42874018118069736972e-3_f64) * t83490 - F::cast_from(0.85748036236139473944e-3_f64) * t70685 + F::cast_from(0.17149607247227894789e-2_f64) * t70689 - t1222 * t5308 * t81194 / F::new(12.0) + t57464 - F::cast_from(0.19055119163586549765e-3_f64) * t57471 - F::cast_from(0.21722835846488666732e-1_f64) * t70469 * t1791 - F::cast_from(0.21722835846488666732e-1_f64) * t21177 * t5320;
    t83502
}
