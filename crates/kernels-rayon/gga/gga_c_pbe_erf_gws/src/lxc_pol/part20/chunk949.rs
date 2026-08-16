//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 949/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk949(t3562: f64, t649: f64, t661: f64, t1621: f64, t1620: f64, t2627: f64, t7130: f64, t1010: f64, t7793: f64, t2615: f64, t2622: f64, t3553: f64) -> (f64, f64, f64, f64, f64) {
    let t10691 = t649 * t3562;
    let t10692 = t10691 * t661;
    let t10693 = t1621 * t10692;
    let t10695 = 4.0_f64 / 15.0_f64 * t1620 * t10693;
    let t10697 = 8.0_f64 / 15.0_f64 * t7130 * t2627;
    let t10699 = 8.0_f64 / 45.0_f64 * t7793 * t1010;
    let t10701 = 16.0_f64 / 45.0_f64 * t2615 * t2622;
    let t10702 = t649 * t3553;
    (t10695, t10697, t10699, t10701, t10702)
}
