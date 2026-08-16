//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 853/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk853(t13368: f64, t6241: f64, t904: f64, t916: f64, t2157: f64, t3854: f64, t3219: f64, t3235: f64, t858: f64, t867: f64, t6240: f64, t3373: f64, t339: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13426 = t13368 * t6241;
    let t13428 = t916 * t904 * t13426;
    let t13431 = t2157 * t3854;
    let t13433 = t3235 * t3219 * t13431;
    let t13437 = t867 * t858 * t13426;
    let t13439 = t6240 * t13437 / 16.0_f64;
    let t13440 = t3373 * t339;
    (t13428, t13431, t13433, t13437, t13439, t13440)
}
