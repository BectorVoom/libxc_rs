//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 275/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk275(t1212: f64, t799: f64, t27: f64, t89: f64, t1188: f64, t791: f64, t788: f64) -> (f64, f64, f64, f64) {
    let t1213 = t799 * t1212;
    let t1215 = t89 * t27 * t1213;
    let t1217 = -t791 - t1188 / 18.0_f64 - t1215 / 6.0_f64;
    let t1218 = t788 * t1217;
    (t1213, t1215, t1217, t1218)
}
