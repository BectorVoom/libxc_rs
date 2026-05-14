//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 664/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk664<F: Float>(t8129: F, t8143: F, t8156: F, t8737: F, t8739: F, t8741: F, t9904: F, t9906: F, t9909: F, t9911: F, t9913: F, t9915: F, t9917: F, t9919: F, t9921: F, t9923: F) -> (F,) {
    let t10457 = -0.42483693136193860285e-2 * t8737 - 0.15965655602485078085e0 * t8739 + 0.10643770401656718724e0 * t8741 + t8129 - 0.5454932330849068346e-1 * t9904 - 0.25401708187682578962e-2 * t9906 - t8143 - 0.19957069503106347607e-1 * t9909 + 0.79656924630363488034e-3 * t9911 - 0.66380770525302906695e-3 * t9913 - 0.19957069503106347607e-1 * t9915 + 0.2993560425465952141e-1 * t9917 - 0.55759847241254441624e-2 * t9919 - 0.11974241701863808564e0 * t9921 - 0.26552308210121162678e-2 * t9923 - t8156;
    (t10457,)
}
