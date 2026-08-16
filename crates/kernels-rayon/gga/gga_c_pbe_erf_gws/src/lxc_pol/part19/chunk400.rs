//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 400/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk400(t1351: f64, t1336: f64, t169: f64, t700: f64, t770: f64, t6: f64, t837: f64) -> (f64, f64, f64, f64) {
    let t1352 = 8.0_f64 * t1351;
    let t1353 = 6.0_f64 * t1336;
    let t1360 = t169 * t770 * t700;
    let t1365 = t6 * t837;
    (t1352, t1353, t1360, t1365)
}
