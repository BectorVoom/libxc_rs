//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1050/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1050(t13290: f64, t816: f64, t13414: f64, t2142: f64, t13353: f64, t11600: f64, t8833: f64, t13173: f64, t2145: f64, t12041: f64, t37701: f64, t3854: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45017 = t816 * t13290;
    let t45048 = t13414 * t2142;
    let t45063 = t13353 * t2142;
    let t45069 = t11600 * t8833;
    let t45074 = t13173 * t2145;
    let t45088 = t12041 * t37701;
    let t45100 = t3854 * param_a_c;
    (t45017, t45048, t45063, t45069, t45074, t45088, t45100)
}
