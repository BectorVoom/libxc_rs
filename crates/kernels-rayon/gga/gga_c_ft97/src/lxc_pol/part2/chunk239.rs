//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 239/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk239(t799: f64, t824: f64, t27: f64, t89: f64, t791: f64, t795: f64, t788: f64) -> (f64, f64, f64, f64) {
    let t825 = t799 * t824;
    let t827 = t89 * t27 * t825;
    let t829 = -t791 - t795 / 18.0_f64 - t827 / 6.0_f64;
    let t830 = t788 * t829;
    (t825, t827, t829, t830)
}
