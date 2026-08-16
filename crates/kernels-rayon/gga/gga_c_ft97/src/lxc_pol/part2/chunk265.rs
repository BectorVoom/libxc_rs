//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 265/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk265(t108: f64, t88: f64, t948: f64, t980: f64, t984: f64, t986: f64, t920: f64) -> (f64, f64) {
    let t989 = -t108 * t948 - t88 * t984 - 2.0_f64 * t980 + 2.0_f64 * t986;
    let t992 = -t920;
    (t989, t992)
}
