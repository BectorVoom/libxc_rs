//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1047/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1047(t13086: f64, t6: f64, t254: f64, t13309: f64, t6203: f64, t13541: f64, t6416: f64, t11668: f64, t11868: f64, t11782: f64, t8824: f64, t1134: f64, t3065: f64, t3854: f64, t858: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44591 = t6 * t13086;
    let t44592 = t254 * t44591;
    let t44600 = t6203 * t13309;
    let t44604 = t6416 * t13541;
    let t44606 = t11668 * t11868;
    let t44629 = t11782 * t8824;
    let t44650 = t3065 * t858 * t1134 * t3854;
    (t44592, t44600, t44604, t44606, t44629, t44650)
}
