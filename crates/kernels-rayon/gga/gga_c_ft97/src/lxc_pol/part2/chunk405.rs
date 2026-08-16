//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 405/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk405(t184: f64, t2235: f64, t5: f64, t619: f64, t171: f64, t360: f64, t70: f64) -> (f64, f64, f64, f64) {
    let t2236 = t2235 * t184;
    let t2240 = t5 * t619;
    let t2247 = 1.0_f64 / t171 / t360;
    let t2248 = t2247 * t70;
    (t2236, t2240, t2247, t2248)
}
