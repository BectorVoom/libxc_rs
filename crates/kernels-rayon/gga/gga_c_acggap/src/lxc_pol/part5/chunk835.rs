//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 835/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk835(t2611: f64, t2617: f64, t2607: f64, t288: f64, t656: f64, t668: f64, t912: f64, t60: f64, t721: f64, t2663: f64, t2738: f64, t244: f64, t2868: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11661 = t2617 * t2611;
    let t11665 = 0.67471172535210825684e-1_f64 * t656 * t2607 * t288;
    let t11668 = 0.86748650402413918736e-1_f64 * t656 * t668 * t912;
    let t11669 = t60 * t721;
    let t11672 = 0.1301229756036208781e0_f64 * t11669 * t2738 * t2663;
    let t11679 = t2868 * t244;
    (t11661, t11665, t11668, t11669, t11672, t11679)
}
