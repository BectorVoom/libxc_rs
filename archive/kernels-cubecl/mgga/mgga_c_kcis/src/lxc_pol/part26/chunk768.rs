//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 768/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk768<F: Float>(t851: F, t8881: F, t2679: F, t843: F, t189: F, t197: F, t2682: F, t8825: F, t237: F, t2658: F, t2663: F, t2666: F, t2676: F, t2684: F, t2695: F, t845: F, t852: F, t859: F, t8630: F, t8646: F, t8649: F, t8653: F, t8666: F, t8674: F, t8725: F, t8737: F, t8745: F, t88: F, t8850: F, t8858: F, t8862: F, t8866: F) -> F {
    let t8882 = t8881 * t851;
    let t8886 = F::cast_from(1.0_f64) / t2679 / t843;
    let t8887 = t189 * t8886;
    let t8889 = F::cast_from(1.0_f64) / t2682 / t197;
    let t8890 = t8825 * t8889;
    let t8893 = F::cast_from(0.32530742648344572643e-1_f64) * t237 * t8850 * t2695 + F::cast_from(0.10274e0_f64) * t237 * t88 * t2663 * t2666 + F::cast_from(0.21687161765563048428e-1_f64) * t237 * t8858 * t859 - F::cast_from(0.16522997748472177549e1_f64) * t237 * t8862 * t2684 + F::cast_from(0.68493333333333333332e-1_f64) * t237 * t8866 * t852 - F::cast_from(0.51369999999999999999e-1_f64) * t237 * t2658 * t2676 - t8745 + t8725 - t8737 - t8646 + t8649 + t8653 - t8674 - t8666 - t8630 + F::cast_from(1.0_f64) * t845 * t8882 + F::cast_from(0.20691336878655965246e4_f64) * t8887 * t8890;
    t8893
}
