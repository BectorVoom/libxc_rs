//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 522/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk522<F: Float>(t1: F, t2299: F, t2182: F, t904: F, t2079: F, t2081: F, t2083: F, t816: F) -> (F, F, F, F) {
    let t2300 = t2299 * t1;
    let t2302 = t2300 * t904 * t2182;
    let t2305 = t2079 * t2081;
    let t2306 = t2083 * t816;
    (t2300, t2302, t2305, t2306)
}
