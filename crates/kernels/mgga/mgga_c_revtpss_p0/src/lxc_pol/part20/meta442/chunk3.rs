//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1690/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1690<F: Float>(t46: F, t47: F, t58: F, t59: F, t2681: F, t64: F, t10326: F, t10345: F, t10355: F, t10357: F, t10360: F, t10361: F, t10364: F, t10368: F, t10372: F, t2251: F, t2258: F, t2270: F, t2275: F, t2276: F, t2279: F, t2282: F, t39443: F, t39449: F, t39457: F, t42748: F, t44: F, t48: F, t49: F, t56: F, t60: F, t614: F, t617: F) -> (F, F) {
    let t46063 = t46 * t46;
    let t46065 = F::new(1.0) / t47 / t46063;
    let t46072 = t58 * t58;
    let t46074 = F::new(1.0) / t59 / t46072;
    let t46089 = t64 * t2681;
    let t46090 = F::new(20944.0) / F::new(81.0) * t46089;
    let t46091 = F::new(10.0) / F::new(9.0) * t56 * t10372 * t10326 - F::new(80.0) / F::new(9.0) * t614 * t10361 - F::new(5.0) / F::new(18.0) * t44 * t10355 * t2251 * t2258 + F::new(5.0) / F::new(6.0) * t44 * t2275 * t39449 + F::new(10.0) / F::new(9.0) * t44 * t10360 * t10326 + F::new(5.0) / F::new(18.0) * t56 * t10368 * t2251 * t2258 + F::new(5.0) / F::new(6.0) * t56 * t2282 * t39449 + F::new(40.0) / F::new(81.0) * t614 * t10357 - F::new(80.0) / F::new(9.0) * t614 * t10364 + F::new(5.0) / F::new(162.0) * t44 * t46065 * t39443 + F::new(5.0) / F::new(6.0) * t44 * t48 * t39457 + F::new(5.0) / F::new(162.0) * t56 * t46074 * t39443 - F::new(5.0) / F::new(6.0) * t56 * t60 * t39457 + F::new(20944.0) / F::new(81.0) * t42748 * t49 - F::new(12320.0) / F::new(81.0) * t10345 * t617 + F::new(440.0) / F::new(9.0) * t2270 * t2279 + F::new(440.0) / F::new(27.0) * t2270 * t2276 - t46090;
    (t46089, t46091)
}
