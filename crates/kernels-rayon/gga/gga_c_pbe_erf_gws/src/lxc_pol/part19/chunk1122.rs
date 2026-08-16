//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1122/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1122(t4116: f64, t945: f64, t1206: f64, t810: f64, t353: f64, t4386: f64, t1205: f64, t2416: f64) -> (f64, f64, f64, f64) {
    let t14161 = t4116 * t945;
    let t14180 = t1206 * t810;
    let t14181 = t353 * t14180;
    let t14182 = t4386 * t14181;
    let t14185 = t2416 * t1205;
    (t14161, t14180, t14182, t14185)
}
