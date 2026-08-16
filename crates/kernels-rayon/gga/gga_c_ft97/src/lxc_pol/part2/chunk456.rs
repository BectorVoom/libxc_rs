//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 456/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk456(t2579: f64, t729: f64, t762: f64, t1882: f64, t726: f64, t684: f64, t724: f64, t773: f64, t2413: f64, t265: f64, t2404: f64, t241: f64) -> (f64, f64, f64, f64, f64) {
    let t2581 = t729 * t762 * t2579;
    let t2584 = t1882 * t726;
    let t2587 = t724 * t773 * t684;
    let t2591 = t724 * t265 * t2413;
    let t2594 = t2404 * t241;
    (t2581, t2584, t2587, t2591, t2594)
}
