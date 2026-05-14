//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 518/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk518<F: Float>(t2083: F, t820: F, t274: F, t814: F, t2255: F, t2190: F, t904: F, t916: F, t899: F, t912: F, t922: F) -> (F, F, F, F, F) {
    let t2278 = t820 * t2083;
    let t2279 = t274 * t814;
    let t2280 = t2278 * t2279;
    let t2281 = t2255 * t2280;
    let t2285 = t916 * t904 * t2190;
    let t2289 = t899 * t912 * t922;
    (t2278, t2279, t2281, t2285, t2289)
}
