//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3140/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3140(t12910: f64, t12916: f64, t24740: f64, t1248: f64, t24633: f64, t1250: f64, t12787: f64, t12856: f64, t17426: f64, t17729: f64, t20292: f64, t24731: f64, t3718: f64, t3720: f64, t4181: f64, t44225: f64, t44578: f64, t5330: f64, t5335: f64, t5343: f64, t57005: f64, t68289: f64, t72326: f64, t72370: f64, t82293: f64, t82469: f64, t82471: f64, t82476: f64, t82481: f64) -> (f64, f64) {
    let t82491 = t12910 * t12916 * t24740;
    let t82493 = t24633 * t1248;
    let t82510 = 0.85748036236139473947e-3_f64 * t82469 - 0.64311027177104605458e-3_f64 * t3718 * t3720 * t82471 * t1250 - 0.64311027177104605458e-3_f64 * t3718 * t3720 * t82476 * t1250 + 0.19055119163586549765e-2_f64 * t17729 * t44225 * t20292 * t82481 + 0.42874018118069736972e-2_f64 * t57005 * t12787 * t68289 * t4181 + 0.85748036236139473947e-3_f64 * t82491 - 0.21437009059034868486e-3_f64 * t3718 * t3720 * t82493 * t1250 + 0.12862205435420921092e-2_f64 * t17426 * t24731 + 0.12862205435420921092e-2_f64 * t44578 * t3720 * t82293 * t12856 + 0.12862205435420921092e-2_f64 * t72370 * t5330 * t5343 - 0.64311027177104605458e-3_f64 * t72326 * t5330 * t5335;
    (t82493, t82510)
}
