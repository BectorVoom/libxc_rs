//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 245/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk245(t192: f64, t824: f64, t852: f64, t462: f64, t847: f64, t849: f64, t92: f64, t845: f64, t91: f64, t790: f64, t795: f64, t827: f64) -> (f64, f64, f64, f64, f64) {
    let t854 = t192 * t852 * t824;
    let t856 = -t847 - t462 * t849 / 3.0_f64 - t92 * t854;
    let t858 = t91 * t845 * t856;
    let t860 = t790 / 9.0_f64;
    let t863 = t858 / 6.0_f64 - t860 - t795 / 9.0_f64 - t827 / 3.0_f64;
    (t854, t856, t858, t860, t863)
}
