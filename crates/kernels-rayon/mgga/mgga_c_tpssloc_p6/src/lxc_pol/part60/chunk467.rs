//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 467/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk467(t5526: f64, t5668: f64, t2770: f64, t5392: f64, t2768: f64, t123: f64, t2775: f64, t882: f64, t5398: f64, t883: f64, t2765: f64, t4335: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5669 = t5526 + t5668;
    let t5677 = t2770 * t5392;
    let t5678 = t2768 * t5677;
    let t5679 = t123 * t5678;
    let t5681 = t2775 * t5392;
    let t5682 = t882 * t5681;
    let t5683 = t123 * t5682;
    let t5685 = t883 * t5398;
    let t5686 = t882 * t5685;
    let t5687 = t123 * t5686;
    let t5689 = t2765 + 0.11872222222222222222e-1_f64 * t4335 - 0.11872222222222222222e-1_f64 * t5679 + 0.35616666666666666666e-1_f64 * t5683 - 0.17808333333333333333e-1_f64 * t5687;
    (t5669, t5677, t5679, t5681, t5683, t5685, t5687, t5689)
}
