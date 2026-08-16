//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1117/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1117(t4110: f64, t810: f64, t2376: f64, t2409: f64, t1205: f64, t2417: f64, t9296: f64, t938: f64, t3067: f64, t338: f64, t4111: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14258 = t4110 * t810;
    let t14260 = t2409 * t2376 * t14258;
    let t14264 = t1205 * t2417;
    let t14266 = t2409 * t9296 * t14264;
    let t14272 = t4110 * t938;
    let t14274 = t2409 * t3067 * t14272;
    let t14280 = t338 * t892 * t4111;
    (t14258, t14260, t14264, t14266, t14272, t14274, t14280)
}
