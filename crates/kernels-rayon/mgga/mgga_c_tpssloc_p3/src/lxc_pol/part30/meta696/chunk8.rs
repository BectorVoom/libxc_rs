//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2235/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2235(t28406: f64, t814: f64, t234: f64, t5631: f64, t6552: f64, t6637: f64, t776: f64, t16758: f64, t22986: f64, t2647: f64, t6646: f64, t5593: f64, t81865: f64) -> (f64, f64, f64, f64) {
    let t98592 = t814 * t28406;
    let t98598 = t234 * t5631;
    let t98601 = t6552 * t6637 * t98598 * t776;
    let t98608 = t22986 * t6646 * t16758 * t2647;
    let t98610 = t81865 * t5593;
    (t98592, t98601, t98608, t98610)
}
