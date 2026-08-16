//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1171/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1171(t14797: f64, t3068: f64, t3990: f64, t3989: f64, t13888: f64, t3060: f64, t9283: f64, t3070: f64, t3965: f64, t2409: f64, t4155: f64, t6781: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14799 = t3990 * t14797 * t3068;
    let t14800 = t3989 * t14799;
    let t14802 = t13888 * t3060;
    let t14803 = t9283 * t14802;
    let t14806 = t3965 * t3070;
    let t14809 = t2409 * t6781 * t4155;
    (t14799, t14800, t14802, t14803, t14806, t14809)
}
