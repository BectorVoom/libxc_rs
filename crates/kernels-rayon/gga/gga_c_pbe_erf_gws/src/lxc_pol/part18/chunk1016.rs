//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1016/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1016(t3886: f64, t938: f64, t2409: f64, t3067: f64, t3742: f64, t6781: f64, t3703: f64, t810: f64) -> (f64, f64, f64, f64) {
    let t11354 = t3886 * t938;
    let t11356 = t2409 * t3067 * t11354;
    let t11360 = t2409 * t6781 * t3742;
    let t11363 = t3703 * t810;
    (t11354, t11356, t11360, t11363)
}
