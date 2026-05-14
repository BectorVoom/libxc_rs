//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 571/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk571<F: Float>(t4395: F, t825: F, t2382: F, t2352: F, t2376: F, t829: F, t830: F, t2358: F, t2387: F, t2083: F, t745: F) -> (F, F, F, F, F) {
    let t4396 = t4395 * t825;
    let t4397 = t2382 * t4396;
    let t4400 = t2376 * t2352;
    let t4402 = t829 * t830 * t4400;
    let t4405 = t2387 * t2358;
    let t4408 = t2083 * t745;
    (t4396, t4397, t4402, t4405, t4408)
}
