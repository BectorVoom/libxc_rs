//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 45/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk45(t95: f64, t102: f64) -> (f64, f64, f64) {
    let t103 = 1.0_f64 / t95;
    let t105 = 0.2923025e1_f64 * t102 * t103;
    let t107 = pow_1_3(9.0_f64);
    let t108 = t107 * t107;
    (t103, t105, t108)
}
