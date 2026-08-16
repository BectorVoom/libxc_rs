//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1054/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1054(t337: f64, t3791: f64, t810: f64, t2147: f64, t3116: f64, t3837: f64, t6501: f64, t11464: f64, t254: f64, t3223: f64, t3765: f64, t6402: f64) -> (f64, f64, f64, f64) {
    let t11841 = t337 * t3791 * t810;
    let t11842 = t2147 * t11841;
    let t11844 = t3116 * t11842 / 48.0_f64;
    let t11846 = t6501 * t3837;
    let t11848 = t254 * t11464;
    let t11849 = t11848 * t3223;
    let t11852 = t6402 * t3765;
    (t11844, t11846, t11849, t11852)
}
