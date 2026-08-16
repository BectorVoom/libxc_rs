//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3248/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3248(t23042: f64, t3915: f64, t686: f64, t72: f64, t22970: f64, t9680: f64, t46368: f64, t46369: f64, t46378: f64, t46385: f64, t46388: f64, t47800: f64, t47802: f64, t47806: f64, t47814: f64, t47835: f64, t47838: f64, t47839: f64, t73623: f64, t73627: f64) -> f64 {
    let t85475 = t3915 * t23042 * t72 * t686;
    let t85480 = t9680 * t22970 * t72 * t686;
    let t85482 = -t46368 + 0.19514881078765566038e-2_f64 * t47800 + 0.51220160311720645768e-1_f64 * t47802 - 0.17073386770573548589e-1_f64 * t46369 - t47806 + 0.21951497276451705328e-1_f64 * t73623 + t47814 + 0.19637199382202157274e-3_f64 * t46378 - 0.58544643236296698113e-1_f64 * t73627 - 0.9757440539382783019e-2_f64 * t85475 - t46385 - t46388 - t47835 - t47838 + 0.43902994552903410658e-1_f64 * t47839 + 0.58544643236296698112e-1_f64 * t85480;
    t85482
}
