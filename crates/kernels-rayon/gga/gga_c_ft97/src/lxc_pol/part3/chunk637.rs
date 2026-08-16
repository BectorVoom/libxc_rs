//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 637/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk637(t6: f64, t78: f64, t694: f64, t373: f64, t929: f64, t1095: f64, t679: f64, t173: f64, t174: f64, t368: f64, t2: f64, t524: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5544 = t78 * t6;
    let t6032 = t694 * t6;
    let t6426 = t373 * t929;
    let t6757 = t679 * t1095;
    let t7239 = t173 * t174;
    let t7240 = t368 * t368;
    let t7241 = 1.0_f64 / t7240;
    let t7242 = t2 * t2;
    let t7367 = t524 * t524;
    (t5544, t6032, t6426, t6757, t7239, t7241, t7242, t7367)
}
