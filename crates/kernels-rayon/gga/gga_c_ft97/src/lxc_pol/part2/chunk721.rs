//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 721/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk721(t11371: f64, t7983: f64, t408: f64, t929: f64, t3020: f64, t428: f64, t388: f64, t939: f64, t398: f64, t401: f64, t51: f64, t6: f64) -> (f64, f64, f64, f64) {
    let t11372 = t7983 * t11371;
    let t11375 = t408 * t929;
    let t11377 = t3020 * t11375 * t428;
    let t11380 = t388 * t939;
    let t11383 = t401 * t6 * t51 * t398;
    (t11372, t11377, t11380, t11383)
}
