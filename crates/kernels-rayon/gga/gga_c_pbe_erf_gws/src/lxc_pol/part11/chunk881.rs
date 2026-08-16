//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 881/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk881(t4687: f64, t4710: f64, t4713: f64, t4717: f64, t4825: f64, t2029: f64, t137: f64, t1478: f64, t1480: f64, t4579: f64, t4585: f64, t6054: f64, t6056: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16369 = 0.4274e0_f64 * t4687;
    let t16370 = 0.28493333333333333333e0_f64 * t4710;
    let t16371 = 0.2137e0_f64 * t4713;
    let t16372 = 0.34366858576436911004e1_f64 * t4717;
    let t16379 = 240.0_f64 * t4825;
    let t16392 = t2029 * t2029;
    let t16393 = 1.0_f64 / t16392;
    let t16394 = t16393 * t137;
    let t16415 = 0.10931146159029059066e-3_f64 * t1478 * t4579 * t1480;
    let t16418 = 0.18276876377896586758e-4_f64 * t6054 * t4585 * t6056;
    (t16369, t16370, t16371, t16372, t16379, t16394, t16415, t16418)
}
