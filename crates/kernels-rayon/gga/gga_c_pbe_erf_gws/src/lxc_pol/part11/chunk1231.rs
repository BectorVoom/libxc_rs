//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1231/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1231(t49463: f64, t2168: f64, t49305: f64, t8599: f64, t44814: f64, t1133: f64, t13290: f64, t343: f64, t28043: f64, t3065: f64, t858: f64, t1105: f64, t6241: f64, param_a_c: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49464 = param_a_c * t49463;
    let t49471 = 3.0_f64 / 4.0_f64 * t2168 * t8599 * t49305;
    let t49472 = 7.0_f64 / 6.0_f64 * t44814;
    let t49474 = t13290 * t1133 * t343;
    let t49478 = t28043 * t3065 * t858 * t49474 / 12.0_f64;
    let t49483 = t6241 * t1105;
    (t49464, t49471, t49472, t49474, t49478, t49483)
}
