//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1097/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1097<F: Float>(t19637: F, t2362: F, t2382: F, t2182: F, t2395: F, t6148: F, t830: F, t1452: F, t2083: F, t825: F, t6154: F, t6778: F) -> (F, F, F, F, F) {
    let t19639 = t2382 * t19637 * t2362;
    let t19641 = t2395 * t2182;
    let t19643 = t6148 * t830 * t19641;
    let t19646 = t2083 * t1452;
    let t19647 = t19646 * t825;
    let t19652 = t2382 * t6154 * t6778;
    (t19639, t19643, t19646, t19647, t19652)
}
