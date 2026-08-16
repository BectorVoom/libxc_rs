//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 491/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk491(t2623: f64, t656: f64, t273: f64, t680: f64, t791: f64, t286: f64, t271: f64, t690: f64, t686: f64, t244: f64, t712: f64, t229: f64, t804: f64) -> (f64, f64, f64, f64, f64) {
    let t2624 = t656 * t2623;
    let t2625 = 0.32530743900905219526e-1_f64 * t2624;
    let t2627 = t791 * t680 * t273;
    let t2628 = t286 * t2627;
    let t2629 = 0.35089341735807877242e1_f64 * t2628;
    let t2631 = t690 * t271;
    let t2632 = t686 * t680 * t2631;
    let t2633 = t286 * t2632;
    let t2634 = 0.51947577317044391277e2_f64 * t2633;
    let t2635 = t712 * t244;
    let t2641 = t229 * t804;
    (t2625, t2629, t2634, t2635, t2641)
}
