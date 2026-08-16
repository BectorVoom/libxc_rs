//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 768/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk768(t851: f64, t8881: f64, t2679: f64, t843: f64, t189: f64, t197: f64, t2682: f64, t8825: f64, t237: f64, t2658: f64, t2663: f64, t2666: f64, t2676: f64, t2684: f64, t2695: f64, t845: f64, t852: f64, t859: f64, t8630: f64, t8646: f64, t8649: f64, t8653: f64, t8666: f64, t8674: f64, t8725: f64, t8737: f64, t8745: f64, t88: f64, t8850: f64, t8858: f64, t8862: f64, t8866: f64) -> f64 {
    let t8882 = t8881 * t851;
    let t8886 = 1.0_f64 / t2679 / t843;
    let t8887 = t189 * t8886;
    let t8889 = 1.0_f64 / t2682 / t197;
    let t8890 = t8825 * t8889;
    let t8893 = 0.32530742648344572643e-1_f64 * t237 * t8850 * t2695 + 0.10274e0_f64 * t237 * t88 * t2663 * t2666 + 0.21687161765563048428e-1_f64 * t237 * t8858 * t859 - 0.16522997748472177549e1_f64 * t237 * t8862 * t2684 + 0.68493333333333333332e-1_f64 * t237 * t8866 * t852 - 0.51369999999999999999e-1_f64 * t237 * t2658 * t2676 - t8745 + t8725 - t8737 - t8646 + t8649 + t8653 - t8674 - t8666 - t8630 + 1.0_f64 * t845 * t8882 + 0.20691336878655965246e4_f64 * t8887 * t8890;
    t8893
}
