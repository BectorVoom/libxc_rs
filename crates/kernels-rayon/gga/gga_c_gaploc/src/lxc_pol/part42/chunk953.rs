//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 953/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk953(t1377: f64, t14348: f64, t14266: f64, t158: f64, t203: f64, t550: f64, t123: f64, t1358: f64, t1365: f64, t44258: f64, t44262: f64, t44263: f64, t44264: f64, t44267: f64, t44278: f64, t44281: f64, t44284: f64, t44288: f64, t44292: f64, t44293: f64, t44298: f64, t46859: f64, t46862: f64, t46865: f64, t488: f64) -> (f64, f64, f64, f64, f64) {
    let t49820 = t1377 * t14348;
    let t49821 = t158 * t14266;
    let t49826 = t203 * t14266;
    let t49827 = t550 * t49826;
    let t49834 = -t44258 + t44262 + t44263 - t44264 - t44267 - t44278 + t44281 - t44284 + t44288 - 0.31616674039640166221e-2_f64 * t1358 * t49821 * t123 * t488 + 0.31616674039640166221e-2_f64 * t1358 * t1365 * t49827 - t44292 - t44293 + 0.47425011059460249332e-2_f64 * t46859 - 0.142275033178380748e-1_f64 * t46862 + 0.94850022118920498664e-2_f64 * t46865 - t44298;
    (t49820, t49821, t49826, t49827, t49834)
}
