//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 657/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk657(t1775: f64, t5106: f64, t458: f64, t5118: f64, t5114: f64, t5092: f64, t9890: f64, t18168: f64, t18171: f64, t18174: f64, t5132: f64, t761: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18305 = t1775 * t5106;
    let t18314 = t458 * t5118;
    let t18316 = t458 * t5114;
    let t18370 = t9890 * t5092;
    let t18381 = t18168 / 9.0_f64;
    let t18382 = 2.0_f64 / 9.0_f64 * t18171;
    let t18383 = 2.0_f64 / 27.0_f64 * t18174;
    let t18391 = t5132 * t761;
    (t18305, t18314, t18316, t18370, t18381, t18382, t18383, t18391)
}
