//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 587/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk587<F: Float>(t135: F, t2191: F, t2194: F, t2201: F, t2237: F, t2245: F, t2325: F, t2327: F, t2330: F, t2334: F, t2338: F, t2342: F, t2457: F, t2461: F, t2464: F, t273: F, t957: F) -> F {
    let t2467 = t135 * t2457 * t273 * t957 - t135 * t2461 * t2464 * t273 - t2191 + t2194 - t2201 + t2237 + t2245 + t2325 + t2327 - t2330 + t2334 - t2338 - t2342;
    t2467
}
