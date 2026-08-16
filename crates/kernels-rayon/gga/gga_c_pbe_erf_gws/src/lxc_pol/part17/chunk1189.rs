//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1189/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1189(t13991: f64, t9270: f64, t4002: f64, t4453: f64, t13939: f64, t2367: f64, t2271: f64, t938: f64, t6745: f64, t13808: f64, t13877: f64, t2242: f64, t4013: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51102 = t9270 * t13991;
    let t51122 = t4453 * t4002;
    let t51126 = t2367 * t13939;
    let t51134 = t2271 * t938;
    let t51142 = t6745 * t4002;
    let t51153 = t13808 * t13877;
    let t51156 = t2242 * t4013;
    (t51102, t51122, t51126, t51134, t51142, t51153, t51156)
}
