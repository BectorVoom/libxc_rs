//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1158/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1158(t12213: f64, t2409: f64, t4097: f64, t4207: f64, t6781: f64, t1205: f64, t8589: f64, t829: f64, t830: f64) -> (f64, f64, f64) {
    let t14902 = t2409 * t12213 * t4097;
    let t14906 = t2409 * t6781 * t4207;
    let t14909 = t8589 * t1205;
    let t14911 = t829 * t830 * t14909;
    (t14902, t14906, t14911)
}
