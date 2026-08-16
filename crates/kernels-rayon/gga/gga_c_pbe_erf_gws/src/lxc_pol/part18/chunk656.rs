//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 656/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk656(t3354: f64, t478: f64, t3629: f64, t3631: f64, t3633: f64) -> (f64, f64) {
    let t3635 = t478 * t3354;
    let t3637 = -t3629 / 9.0_f64 + t3631 / 3.0_f64 - t3633 / 9.0_f64 + t3635 / 3.0_f64;
    (t3635, t3637)
}
