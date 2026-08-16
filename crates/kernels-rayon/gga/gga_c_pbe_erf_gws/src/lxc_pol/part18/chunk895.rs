//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 895/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk895(t2409: f64, t2501: f64, t3189: f64, t3744: f64, t4414: f64, t2366: f64, t3916: f64, t833: f64, t3039: f64, t3920: f64, t3909: f64, t840: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9948 = t2409 * t2501 * t3189;
    let t9953 = t4414 * t3744;
    let t9955 = t3916 * t2366;
    let t9956 = t9955 * t833;
    let t9958 = t3039 * t3920;
    let t9962 = t840 * t3909;
    (t9948, t9953, t9955, t9956, t9958, t9962)
}
