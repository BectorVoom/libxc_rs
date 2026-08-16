//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1167/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1167(t2345: f64, t2660: f64, t88239: f64, t89: f64, t1091: f64, t22199: f64, t10248: f64, t446: f64, t22386: f64, t3690: f64, t10409: f64, t3699: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89802 = t89 * t2345 * t2660 * t88239;
    let t89805 = t1091 * t22199;
    let t89807 = t446 * t10248 * t89805;
    let t89809 = t3690 * t22386;
    let t89811 = t446 * t10409 * t89809;
    let t89813 = t3699 * t22386;
    (t89802, t89805, t89807, t89809, t89811, t89813)
}
