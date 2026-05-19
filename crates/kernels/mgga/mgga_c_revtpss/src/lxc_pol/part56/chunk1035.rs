//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1035/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1035<F: Float>(t119903: F, t31838: F, t31840: F, t845: F, t31834: F, t846: F, t119808: F, t7063: F, t31801: F, t2769: F, t31777: F, t786: F) -> (F, F, F, F, F, F) {
    let t119904 = F::cast_from(0.18822977838986977999e-5_f64) * t119903;
    let t119912 = t31838 * t845 * t31840;
    let t119914 = t31834 * t846;
    let t119919 = t7063 * t119808;
    let t119920 = t119919 * t31801;
    let t119927 = t31777 * t2769;
    let t119928 = t786 * t119927;
    (t119904, t119912, t119914, t119920, t119927, t119928)
}
