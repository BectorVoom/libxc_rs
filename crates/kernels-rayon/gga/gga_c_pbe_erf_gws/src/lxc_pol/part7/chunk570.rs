//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 570/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk570(t4344: f64, t116: f64, t784: f64, t799: f64, t798: f64, t259: f64, t47: f64, t1403: f64, t418: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4345 = 0.1232289865202e1_f64 * t4344;
    let t4347 = t799 * t784 * t116;
    let t4348 = t798 * t4347;
    let t4349 = 0.18256146151140740741e1_f64 * t4348;
    let t4351 = 1.0_f64 / t47 / t259;
    let t4352 = t1403 * t418;
    (t4345, t4347, t4348, t4349, t4351, t4352)
}
