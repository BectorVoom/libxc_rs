//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3140/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3140<F: Float>(t12910: F, t12916: F, t24740: F, t1248: F, t24633: F, t1250: F, t12787: F, t12856: F, t17426: F, t17729: F, t20292: F, t24731: F, t3718: F, t3720: F, t4181: F, t44225: F, t44578: F, t5330: F, t5335: F, t5343: F, t57005: F, t68289: F, t72326: F, t72370: F, t82293: F, t82469: F, t82471: F, t82476: F, t82481: F) -> (F, F) {
    let t82491 = t12910 * t12916 * t24740;
    let t82493 = t24633 * t1248;
    let t82510 = F::cast_from(0.85748036236139473947e-3_f64) * t82469 - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t3720 * t82471 * t1250 - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t3720 * t82476 * t1250 + F::cast_from(0.19055119163586549765e-2_f64) * t17729 * t44225 * t20292 * t82481 + F::cast_from(0.42874018118069736972e-2_f64) * t57005 * t12787 * t68289 * t4181 + F::cast_from(0.85748036236139473947e-3_f64) * t82491 - F::cast_from(0.21437009059034868486e-3_f64) * t3718 * t3720 * t82493 * t1250 + F::cast_from(0.12862205435420921092e-2_f64) * t17426 * t24731 + F::cast_from(0.12862205435420921092e-2_f64) * t44578 * t3720 * t82293 * t12856 + F::cast_from(0.12862205435420921092e-2_f64) * t72370 * t5330 * t5343 - F::cast_from(0.64311027177104605458e-3_f64) * t72326 * t5330 * t5335;
    (t82493, t82510)
}
