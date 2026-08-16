//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1046/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1046(t3854: f64, t8884: f64, t3065: f64, t858: f64, t11667: f64, t3916: f64, t13580: f64, t2142: f64, t13524: f64, t8978: f64, t9246: f64, t13423: f64, t6416: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44477 = t8884 * t3854;
    let t44479 = t3065 * t858 * t44477;
    let t44530 = t3916 * t11667;
    let t44537 = t13580 * t2142;
    let t44577 = t8978 * t9246 * t13524;
    let t44589 = t6416 * t13423;
    (t44477, t44479, t44530, t44537, t44577, t44589)
}
