//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 620/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk620(t2804: f64, t981: f64, t2769: f64, t2771: f64, t2778: f64, t373: f64, t978: f64, t991: f64, t993: f64, t375: f64, t198: f64, t2475: f64, t2478: f64, t2485: f64, t2528: f64, t2536: f64, t2626: f64, t2628: f64, t2631: f64, t2635: f64, t2639: f64, t2643: f64, t330: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2805 = t981 * t2804;
    let t2807 = t2769 * t373 - 2.0_f64 * t2771 * t991 + 2.0_f64 * t2778 * t978 - t2805 * t978;
    let t2811 = t993 * t993;
    let t2813 = t375 * t375;
    let t2814 = 1.0_f64 / t2813;
    let t2817 = t198 * t2807 * t330 * t995 - t198 * t2811 * t2814 * t330 - t2475 + t2478 - t2485 + t2528 + t2536 + t2626 + t2628 - t2631 + t2635 - t2639 - t2643;
    (t2805, t2807, t2811, t2813, t2814, t2817)
}
