//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2025/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2025<F: Float>(t112: F, t843: F, t239: F, t655: F, t665: F, t2339: F, t624: F, t10208: F, t68: F, t25081: F, t7234: F, t10368: F, t55: F) -> (F, F, F, F, F, F, F) {
    let t94973 = t843 * t112;
    let t94974 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t94973;
    let t94975 = t239 * t655;
    let t94976 = t94975 * t665;
    let t94978 = t624 * t2339;
    let t94982 = t68 * t10208;
    let t95088 = t7234 * t25081;
    let t96733 = t55 * t10368;
    (t94974, t94975, t94976, t94978, t94982, t95088, t96733)
}
