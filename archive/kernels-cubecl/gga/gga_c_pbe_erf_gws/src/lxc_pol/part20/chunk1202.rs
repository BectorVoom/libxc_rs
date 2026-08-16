//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1202/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1202<F: Float>(t3139: F, t332: F, t2118: F, t8913: F, t4395: F, t814: F, t2306: F, t810: F, t274: F, t4408: F, t858: F, t892: F) -> (F, F, F, F, F, F) {
    let t27729 = t332 * t3139;
    let t28139 = t2118 * t8913;
    let t28652 = t4395 * t814;
    let t28657 = t2306 * t810;
    let t29260 = t4408 * t274;
    let t29751 = t858 * t892;
    (t27729, t28139, t28652, t28657, t29260, t29751)
}
