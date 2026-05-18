//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 798/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk798<F: Float>(t2157: F, t343: F, t2306: F, t346: F, t2251: F, t933: F, t2250: F, t810: F, t2365: F, t885: F, t2149: F, t4395: F) -> (F, F, F, F, F, F, F) {
    let t6241 = t2157 * t343;
    let t6252 = t2306 * t346;
    let t6274 = t2251 * t933;
    let t6275 = t2250 * t6274;
    let t6287 = t2157 * t810;
    let t6331 = t2365 * t885;
    let t6332 = t6331 * t2149;
    let t6335 = t4395 * t346;
    (t6241, t6252, t6275, t6287, t6331, t6332, t6335)
}
