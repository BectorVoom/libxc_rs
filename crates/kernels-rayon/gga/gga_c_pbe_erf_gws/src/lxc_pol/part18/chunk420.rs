//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 420/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk420(t732: f64, t735: f64, t155: f64, t266: f64, t265: f64, t586: f64, t615: f64) -> (f64, f64, f64, f64) {
    let t1615 = t732 * t735;
    let t1617 = t266 * t155;
    let t1619 = 2.0_f64 / 135.0_f64 * t265 * t1617;
    let t1620 = t615 * t586;
    (t1615, t1617, t1619, t1620)
}
