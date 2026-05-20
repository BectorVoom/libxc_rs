//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3248/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3248<F: Float>(t23042: F, t3915: F, t686: F, t72: F, t22970: F, t9680: F, t46368: F, t46369: F, t46378: F, t46385: F, t46388: F, t47800: F, t47802: F, t47806: F, t47814: F, t47835: F, t47838: F, t47839: F, t73623: F, t73627: F) -> F {
    let t85475 = t3915 * t23042 * t72 * t686;
    let t85480 = t9680 * t22970 * t72 * t686;
    let t85482 = -t46368 + F::cast_from(0.19514881078765566038e-2_f64) * t47800 + F::cast_from(0.51220160311720645768e-1_f64) * t47802 - F::cast_from(0.17073386770573548589e-1_f64) * t46369 - t47806 + F::cast_from(0.21951497276451705328e-1_f64) * t73623 + t47814 + F::cast_from(0.19637199382202157274e-3_f64) * t46378 - F::cast_from(0.58544643236296698113e-1_f64) * t73627 - F::cast_from(0.9757440539382783019e-2_f64) * t85475 - t46385 - t46388 - t47835 - t47838 + F::cast_from(0.43902994552903410658e-1_f64) * t47839 + F::cast_from(0.58544643236296698112e-1_f64) * t85480;
    t85482
}
