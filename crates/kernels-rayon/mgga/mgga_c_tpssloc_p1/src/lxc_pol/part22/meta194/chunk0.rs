//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1142/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1142(t5705: f64, t894: f64, t2815: f64, t5698: f64, t901: f64, t2826: f64, t5677: f64, t136: f64, t5681: f64, t908: f64, t5685: f64, t2810: f64, t2823: f64, t4335: f64, t4384: f64, t5679: f64, t5683: f64, t5687: f64, t5699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5706 = t894 * t5705;
    let t5712 = t2815 * t5698;
    let t5714 = t901 * t5705;
    let t5717 = t2826 * t5677;
    let t5718 = t136 * t5717;
    let t5720 = t908 * t5681;
    let t5721 = t136 * t5720;
    let t5723 = t908 * t5685;
    let t5724 = t136 * t5723;
    let t5726 = -0.9494625e0_f64 * t5699 + 0.1898925e1_f64 * t5706 + t2810 + 0.19931111111111111111e0_f64 * t4335 - 0.19931111111111111111e0_f64 * t5679 + 0.59793333333333333334e0_f64 * t5683 - 0.29896666666666666667e0_f64 * t5687 + 0.15358125e0_f64 * t5712 + 0.3071625e0_f64 * t5714 + t2823 + 0.10954222222222222222e0_f64 * t4384 - 0.27385555555555555556e-1_f64 * t5718 + 0.16431333333333333333e0_f64 * t5721 - 0.82156666666666666667e-1_f64 * t5724;
    (t5706, t5712, t5714, t5717, t5718, t5720, t5721, t5723, t5724, t5726)
}
