//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 505/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk505(t2607: f64, t31: f64, t4: f64, t195: f64, t682: f64, t656: f64, t691: f64, t243: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2609 = t4 * t2607 * t31;
    let t2610 = 0.34450798614814814813e-2_f64 * t2609;
    let t2611 = t195 * t682;
    let t2612 = t656 * t2611;
    let t2613 = 0.16265371950452609763e-1_f64 * t2612;
    let t2614 = t195 * t691;
    let t2615 = t656 * t2614;
    let t2616 = 0.48159733137676571078e0_f64 * t2615;
    let t2617 = t243 * t4;
    (t2609, t2610, t2611, t2612, t2613, t2614, t2615, t2616, t2617)
}
