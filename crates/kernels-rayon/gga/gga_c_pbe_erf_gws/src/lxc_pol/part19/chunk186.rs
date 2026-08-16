//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 186/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk186(t120: f64, t133: f64, t156: f64, t488: f64, t491: f64, t498: f64) -> (f64, f64) {
    let t517 = 0.28737583333333333333e0_f64 * t133 * t156 * t120;
    let t520 = -t488 - t491 - t517 - 0.1724255e1_f64 * t133 * t498;
    (t517, t520)
}
