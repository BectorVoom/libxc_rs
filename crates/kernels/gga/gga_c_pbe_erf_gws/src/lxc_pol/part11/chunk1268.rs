//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1268/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1268<F: Float>(t1109: F, t11514: F, t2255: F, t2277: F, t29599: F, t3235: F, t3258: F, t3373: F, t46253: F, t46280: F, t50069: F, t50187: F, t50189: F, t50193: F, t50201: F, t50206: F, t50207: F, t9425: F) -> F {
    let t50208 = -F::new(3.0) / F::new(64.0) * t9425 * t3235 * t11514 * t50069 - t50187 - F::new(7.0) / F::new(48.0) * t46253 + t50189 + t50193 - t2277 * t2255 * t3258 * t3373 * t1109 / F::new(512.0) + F::new(7.0) / F::new(96.0) * t46280 - t50201 + F::new(595.0) / F::new(1296.0) * t29599 - t50206 - t50207;
    t50208
}
