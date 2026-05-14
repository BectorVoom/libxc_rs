//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1108/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1108<F: Float>(t46251: F, t28074: F, t21328: F, t50002: F, t858: F, t884: F, t11600: F, t11808: F, t50019: F, t866: F, t867: F, t46324: F, t1109: F, t11514: F, t2255: F, t2277: F, t29599: F, t3235: F, t3258: F, t3373: F, t46253: F, t46280: F, t50069: F, t9425: F) -> (F, F, F, F, F, F, F) {
    let t50187 = 7.0 / 12.0 * t46251;
    let t50189 = 455.0 / 162.0 * t28074;
    let t50193 = 5.0 / 4.0 * t884 * t21328 * t858 * t50002;
    let t50201 = t11600 * t11808 / 8.0;
    let t50206 = t866 * t867 * t858 * t50019 / 96.0;
    let t50207 = 7.0 / 12.0 * t46324;
    let t50208 = -3.0 / 64.0 * t9425 * t3235 * t11514 * t50069 - t50187 - 7.0 / 48.0 * t46253 + t50189 + t50193 - t2277 * t2255 * t3258 * t3373 * t1109 / 512.0 + 7.0 / 96.0 * t46280 - t50201 + 595.0 / 1296.0 * t29599 - t50206 - t50207;
    (t50187, t50189, t50193, t50201, t50206, t50207, t50208)
}
