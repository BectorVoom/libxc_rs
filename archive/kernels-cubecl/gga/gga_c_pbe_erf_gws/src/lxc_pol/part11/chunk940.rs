//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 940/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk940<F: Float>(t328: F, t6552: F, t331: F, t863: F, t19: F, t2298: F, t56: F, t1: F, t16192: F, t191: F, t2251: F, t2276: F, t6383: F) -> (F, F, F, F) {
    let t21296 = t6552 * t328;
    let t21298 = t863 * t21296 * t331;
    let t21328 = t56 * t2298 * t19;
    let t21361 = t191 * t16192 * t1;
    let t21399 = t2276 * t2251 * t6383;
    (t21298, t21328, t21361, t21399)
}
