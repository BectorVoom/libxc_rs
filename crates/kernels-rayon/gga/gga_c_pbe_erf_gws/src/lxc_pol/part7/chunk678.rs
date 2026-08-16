//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 678/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk678(t703: f64, t713: f64, t712: f64, t247: f64, t4562: f64, t251: f64, t1906: f64, t719: f64, t256: f64, t1354: f64, t19: f64, t336: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5441 = t703 * t713;
    let t5443 = 0.13506172839506172839e-1_f64 * t712 * t5441;
    let t5444 = t4562 * t247;
    let t5445 = t5444 * t251;
    let t5448 = t1906 * t719;
    let t5449 = t5448 * t256;
    let t5450 = t1354 * t19;
    let t5451 = t5450 * t336;
    (t5441, t5443, t5444, t5445, t5448, t5449, t5450, t5451)
}
