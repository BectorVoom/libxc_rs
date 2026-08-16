//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2044/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2044<F: Float>(t3268: F, t7143: F, t3057: F, t25460: F, t25698: F, t1035: F, t25586: F, t93484: F, t994: F, t1071: F, t7150: F, t8521: F) -> (F, F, F, F, F, F) {
    let t93920 = t7143 * t3268;
    let t93921 = t3057 * t93920;
    let t93928 = t25698 * t25460;
    let t93939 = t1035 * t25586;
    let t93959 = t994 * t93484;
    let t93962 = t7150 * t1071;
    let t93963 = t93962 * t8521;
    (t93920, t93921, t93928, t93939, t93959, t93963)
}
