//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1312/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1312(t12025: f64, t51421: f64, t11996: f64, t14007: f64, t11455: f64, t14092: f64, t14538: f64, t11652: f64, t14498: f64, t14064: f64, t3783: f64, t11820: f64, t14011: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57004 = t51421 * t12025;
    let t57006 = t14007 * t11996;
    let t57009 = t14538 * t14092 * t11455;
    let t57011 = t14498 * t11652;
    let t57013 = t3783 * t14064;
    let t57015 = t14011 * t11820;
    (t57004, t57006, t57009, t57011, t57013, t57015)
}
