//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 799/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk799<F: Float>(t5: F, t745: F, t337: F, t2121: F, t810: F, t816: F, t274: F, t1: F, t2298: F, t253: F, t320: F, t368: F) -> (F, F, F, F, F, F, F) {
    let t6340 = t5 * t745;
    let t6341 = t337 * t6340;
    let t6342 = t2121 * t6341;
    let t6345 = t816 * t810;
    let t6355 = t745 * t274;
    let t6365 = t2298 * t1;
    let t6366 = t6365 * t253;
    let t6382 = F::cast_from(1.0_f64) / t368 / t320;
    (t6341, t6342, t6345, t6355, t6365, t6366, t6382)
}
