//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 806/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk806<F: Float>(t1952: F, t4579: F, t553: F, t1971: F, t4585: F, t5697: F, t6055: F, t1368: F, t19: F, t1339: F, t1331: F, t8: F, t147: F, t551: F, t6041: F, t6047: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16441 = 0.39507780657818961764e-1 * t1952 * t4579 * t553;
    let t16444 = 0.13871971944573393855e-1 * t5697 * t4585 * t1971;
    let t16446 = 0.2267957317922316773e-1 * t6055 * t1971;
    let t16451 = t1368 * t19;
    let t16454 = 0.29725654166942986832e-2 * t1339 * t16451 * t1971;
    let t16463 = 1.0 / t8 / t1331;
    let t16465 = t16463 * t147 * t551;
    let t16467 = 0.74395492895254307406e-5 * t16465 * t553;
    let t16471 = 0.1035981803916141664e0 * t6041 * t553;
    let t16480 = 0.12408369628826103546e0 * t6047 * t553;
    (t16441, t16444, t16446, t16451, t16454, t16463, t16465, t16467, t16471, t16480)
}
