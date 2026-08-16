//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1343/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1343(t18392: f64, t7345: f64, t27617: f64, t4993: f64, t28525: f64, t461: f64, t7324: f64, t210: f64, t29584: f64, t27683: f64, t27710: f64, t27700: f64, t95588: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104371 = t7345 * t18392;
    let t104375 = t27617 * t4993;
    let t104387 = t7324 * t28525 * t461;
    let t104410 = t29584 * t210;
    let t104413 = t27710 * t27683;
    let t104425 = t95588 * t27700;
    (t104371, t104375, t104387, t104410, t104413, t104425)
}
