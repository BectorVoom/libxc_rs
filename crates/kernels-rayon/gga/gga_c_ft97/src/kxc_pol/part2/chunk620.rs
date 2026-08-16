//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 620/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk620(t213: f64, t51: f64, t1109: f64, t6: f64, t694: f64, t373: f64, t929: f64, t237: f64, t173: f64, t174: f64, t368: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4951 = t51 * t213;
    let t4952 = t4951 * t1109;
    let t6032 = t694 * t6;
    let t6426 = t373 * t929;
    let t6783 = t237 * t6;
    let t7239 = t173 * t174;
    let t7240 = t368 * t368;
    let t7241 = 1.0_f64 / t7240;
    let t7242 = t2 * t2;
    (t4952, t6032, t6426, t6783, t7239, t7241, t7242)
}
