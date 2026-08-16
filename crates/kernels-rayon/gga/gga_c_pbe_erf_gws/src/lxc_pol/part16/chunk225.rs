//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 225/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk225(t627: f64, t657: f64, t25: f64, t629: f64, t651: f64, t655: f64) -> (f64, f64) {
    let t658 = t657 * t627;
    let t661 = -t651 - 0.35991666666666666667e-1_f64 * t629 - t655 - 0.66666666666666666667e-2_f64 * t25 * t658;
    (t658, t661)
}
