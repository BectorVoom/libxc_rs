//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 805/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk805<F: Float>(t4806: F, t4814: F, t4687: F, t4710: F, t4713: F, t4717: F, t4825: F, t2029: F, t137: F, t1478: F, t1480: F, t4579: F, t4585: F, t6054: F, t6056: F, t1378: F, t147: F, t2331: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16366 = 0.14035736153892489771e2 * t4806;
    let t16368 = 0.22787712934626154593e-2 * t4814;
    let t16369 = 0.4274e0 * t4687;
    let t16370 = 0.28493333333333333333e0 * t4710;
    let t16371 = 0.2137e0 * t4713;
    let t16372 = 0.34366858576436911004e1 * t4717;
    let t16379 = 240.0 * t4825;
    let t16392 = t2029 * t2029;
    let t16393 = 1.0 / t16392;
    let t16394 = t16393 * t137;
    let t16415 = 0.10931146159029059066e-3 * t1478 * t4579 * t1480;
    let t16418 = 0.18276876377896586758e-4 * t6054 * t4585 * t6056;
    let t16422 = 0.67015213385620818113e-4 * t2331 * t147 * t1378 * t6056;
    (t16366, t16368, t16369, t16370, t16371, t16372, t16379, t16394, t16415, t16418, t16422)
}
