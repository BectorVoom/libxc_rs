//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1006/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1006(t37406: f64, t85469: f64, t7954: f64, t92: f64, t85483: f64, t7763: f64, t1642: f64, t85491: f64, t1557: f64, t85451: f64, t85465: f64, t7800: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t85516 = t37406 * t85469;
    let t85518 = t92 * t7954 * t85516;
    let t85522 = t92 * t7954 * t85483;
    let t85524 = t7763 * t85469;
    let t85526 = t92 * t1642 * t85524;
    let t85529 = t92 * t1642 * t85491;
    let t85531 = t1557 * t85451;
    let t85533 = t92 * t1642 * t85531;
    let t85536 = t92 * t1642 * t85465;
    let t85538 = t7800 * t85469;
    (t85516, t85518, t85522, t85524, t85526, t85529, t85531, t85533, t85536, t85538)
}
