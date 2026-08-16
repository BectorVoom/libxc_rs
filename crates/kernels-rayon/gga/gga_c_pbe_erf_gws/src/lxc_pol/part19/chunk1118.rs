//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1118/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1118(t14079: f64, t918: f64, t1477: f64, t326: f64, t346: f64, t1185: f64, t4021: f64, t828: f64) -> (f64, f64, f64, f64) {
    let t14080 = t14079 * t918;
    let t14081 = 7.0_f64 / 576.0_f64 * t14080;
    let t14083 = t326 * t346 * t1477;
    let t14084 = t14083 * t1185;
    let t14092 = t4021 * t828;
    (t14081, t14083, t14084, t14092)
}
