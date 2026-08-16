//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 943/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk943(t3492: f64, t586: f64, t645: f64, t2654: f64, t5390: f64, t3603: f64, t735: f64, t3342: f64, t476: f64, t3346: f64, t92: f64, t3351: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10629 = t3492 * t586;
    let t10631 = 8.0_f64 / 45.0_f64 * t10629 * t645;
    let t10633 = 0.2e-20_f64 * t2654 * t5390;
    let t10634 = t3603 * t735;
    let t10636 = t476 * t3342;
    let t10641 = t92 * t3346;
    let t10646 = t478 * t3351;
    (t10631, t10633, t10634, t10636, t10641, t10646)
}
