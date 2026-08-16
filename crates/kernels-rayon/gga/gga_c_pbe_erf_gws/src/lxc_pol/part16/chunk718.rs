//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 718/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk718(t4347: f64, t798: f64, t259: f64, t47: f64, t34: f64, t39: f64, t261: f64, t52: f64, t56: f64, t825: f64) -> (f64, f64, f64, f64, f64) {
    let t4348 = t798 * t4347;
    let t4349 = 0.18256146151140740741e1_f64 * t4348;
    let t4351 = 1.0_f64 / t47 / t259;
    let t4358 = t34 * t39;
    let t4366 = 1.0_f64 / t52 / t261;
    let t4383 = t825 * t56;
    (t4349, t4351, t4358, t4366, t4383)
}
