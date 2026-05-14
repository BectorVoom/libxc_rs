//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 737/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk737<F: Float>(t1609: F, t8: F, t1613: F, t5585: F, t6010: F, t681: F, t1403: F, t1424: F, t7514: F, t2373: F, t263: F, t193: F, t2459: F, t6008: F, t2371: F, t6061: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t22532 = t8 * t1609;
    let t22794 = t5585 * t1613;
    let t24178 = t681 * t6010;
    let t24179 = t1403 * t24178;
    let t24181 = t7514 * t1424;
    let t24182 = t263 * t2373;
    let t24183 = t24181 * t24182;
    let t24184 = t193 * t24183;
    let t24186 = t263 * t2459;
    let t24187 = t6008 * t24186;
    let t24188 = t193 * t24187;
    let t24191 = t2371 * t6061;
    (t22532, t22794, t24178, t24179, t24181, t24182, t24183, t24184, t24186, t24187, t24188, t24191)
}
