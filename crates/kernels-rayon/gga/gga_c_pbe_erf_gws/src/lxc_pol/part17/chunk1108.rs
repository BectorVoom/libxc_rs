//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1108/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1108(t14031: f64, t2259: f64, t366: f64, t6238: f64, t899: f64, t2268: f64, t2173: f64, t4028: f64, t1184: f64, t2216: f64, t4033: f64, t888: f64) -> (f64, f64, f64, f64, f64) {
    let t14032 = t14031 * t2259;
    let t14035 = t899 * t6238 * t366;
    let t14036 = t14035 * t2268;
    let t14038 = t4028 * t2173;
    let t14040 = t1184 * t2216;
    let t14042 = t4033 * t888;
    (t14032, t14036, t14038, t14040, t14042)
}
