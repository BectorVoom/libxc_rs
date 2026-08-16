//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 855/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk855(t11811: f64, t11817: f64, t11984: f64, t3793: f64, t13368: f64, t343: f64, t858: f64, t867: f64, t866: f64, t13431: f64, t3131: f64, t3139: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13456 = 7.0_f64 / 96.0_f64 * t11811;
    let t13457 = 7.0_f64 / 96.0_f64 * t11817;
    let t13459 = t11984 * t3793 / 32.0_f64;
    let t13461 = t13368 * t343;
    let t13463 = t867 * t858 * t13461;
    let t13465 = t866 * t13463 / 96.0_f64;
    let t13468 = t3139 * t3131 * t13431;
    (t13456, t13457, t13459, t13461, t13463, t13465, t13468)
}
