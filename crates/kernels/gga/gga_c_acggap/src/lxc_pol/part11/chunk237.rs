//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 237/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk237<F: Float>(t909: F, t272: F, t687: F, t791: F, t286: F, t659: F, t706: F, t711: F, t714: F, t717: F, t753: F, t757: F, t774: F, t782: F, t809: F, t150: F, t908: F) -> (F, F, F) {
    let t910 = 0.19751673498613801407e-1 * t909;
    let t912 = t791 * t687 * t272;
    let t913 = t286 * t912;
    let t914 = 0.11696447245269292414e1 * t913;
    let t915 = t711 + t714 - t717 - t753 + t910 + t774 + t782 + t659 + t809 + t914 - t706 - t757;
    let t917 = (t908 + t915) * t150;
    (t912, t914, t917)
}
