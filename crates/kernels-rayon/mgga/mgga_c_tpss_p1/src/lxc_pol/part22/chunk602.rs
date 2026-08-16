//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 602/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk602(t140: f64, t930: f64, t925: f64, t1992: f64, t929: f64, t926: f64, t265: f64, t836: f64, t2459: f64, t1985: f64, t2475: f64, t2478: f64, t2485: f64, t2528: f64, t2536: f64, t2626: f64, t2628: f64, t2631: f64, t2635: f64, t2639: f64, t2643: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2689 = t140 * t930;
    let t2690 = t925 * t2689;
    let t2692 = t929 * t1992;
    let t2693 = t926 * t2692;
    let t2697 = 1.0_f64 / t265 / t836;
    let t2698 = t2697 * t2459;
    let t2699 = t2698 * t1985;
    let t2700 = t926 * t2699;
    let t2703 = -t2475 + t2478 - t2485 + t2528 + t2536 + t2626 + t2628 - t2631 + t2635 - t2639 - t2643;
    (t2689, t2690, t2692, t2693, t2697, t2699, t2700, t2703)
}
