//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1238/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1238(t45048: f64, t13525: f64, t37994: f64, t11414: f64, t37286: f64, t45063: f64, t3180: f64, t45074: f64, t45069: f64, t11478: f64, t2168: f64, t3139: f64, t3855: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49577 = 7.0_f64 / 24.0_f64 * t45048;
    let t49579 = t37994 * t13525 / 8.0_f64;
    let t49581 = t37286 * t11414 / 4.0_f64;
    let t49585 = 7.0_f64 / 24.0_f64 * t45063;
    let t49588 = t45074 * t3180 / 12.0_f64;
    let t49594 = 7.0_f64 / 12.0_f64 * t45069;
    let t49607 = t2168 * t3139 * t11478 * t3855 / 16.0_f64;
    (t49577, t49579, t49581, t49585, t49588, t49594, t49607)
}
