//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 507/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk507(t2631: f64, t680: f64, t686: f64, t286: f64, t244: f64, t712: f64, t811: f64, t814: f64, t229: f64, t804: f64, t243: f64, t803: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2632 = t686 * t680 * t2631;
    let t2633 = t286 * t2632;
    let t2634 = 0.51947577317044391277e2_f64 * t2633;
    let t2635 = t712 * t244;
    let t2637 = t811 * t814;
    let t2641 = t229 * t804;
    let t2642 = 12.0_f64 * t2641;
    let t2643 = t243 * t803;
    (t2632, t2633, t2634, t2635, t2637, t2641, t2642, t2643)
}
