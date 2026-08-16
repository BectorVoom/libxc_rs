//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 666/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk666(t3703: f64, t376: f64, t353: f64, t338: f64, t1105: f64, t1161: f64) -> (f64, f64, f64, f64) {
    let t3737 = t376 * t3703;
    let t3738 = t353 * t3737;
    let t3739 = t338 * t3738;
    let t3742 = t1105 * t1161;
    (t3737, t3738, t3739, t3742)
}
