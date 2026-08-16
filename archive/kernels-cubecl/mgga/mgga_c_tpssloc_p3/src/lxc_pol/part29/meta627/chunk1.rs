//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2071/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2071<F: Float>(t2132: F, t24746: F, t86197: F, t1170: F, t2121: F, t24611: F, t225: F, t24871: F, t2122: F, t7319: F, t24574: F, t24597: F) -> (F, F, F, F, F) {
    let t86368 = t2132 * t86197 * t24746;
    let t86390 = t2121 * t1170 * t24611;
    let t86400 = t24871 * t225;
    let t86403 = t7319 * t2122;
    let t86409 = t24574 * t24597;
    (t86368, t86390, t86400, t86403, t86409)
}
