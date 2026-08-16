//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1057/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1057(t1134: f64, t3206: f64, t3703: f64, t858: f64, t13265: f64, t6402: f64, t13557: f64, t2289: f64, t13126: f64, t21597: f64, t3123: f64, t38063: f64) -> (f64, f64, f64, f64, f64) {
    let t45660 = t3206 * t858 * t1134 * t3703;
    let t45703 = t6402 * t13265;
    let t45741 = t2289 * t13557;
    let t45750 = t13126 * t21597;
    let t45753 = t3123 * t38063;
    (t45660, t45703, t45741, t45750, t45753)
}
