//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1273/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1273(t13793: f64, t56112: f64, t3723: f64, t859: f64, t13792: f64, t3738: f64, t8599: f64, t11660: f64, t331: f64, t3802: f64, t6472: f64, t833: f64) -> (f64, f64, f64, f64) {
    let t56113 = t56112 * t13793;
    let t56115 = t859 * t3723;
    let t56116 = t13792 * t56115;
    let t56118 = t8599 * t3738;
    let t56119 = t13792 * t56118;
    let t56124 = t11660 * t6472 * t3802 * t331 * t833;
    (t56113, t56116, t56119, t56124)
}
