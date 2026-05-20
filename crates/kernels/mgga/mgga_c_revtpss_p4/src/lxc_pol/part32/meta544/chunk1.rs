//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1857/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1857<F: Float>(t10073: F, t2066: F, t25390: F, t886: F, t7058: F, t95730: F, t2439: F, t26434: F, t887: F, t2471: F, t26563: F, t10985: F, t26576: F) -> (F, F, F, F, F) {
    let t95911 = t10073 * t25390 * t2066 * t886;
    let t95914 = F::cast_from(0.22487184191643109717e-1_f64) * t7058 * t95730;
    let t95925 = t2439 * t26434 * t887;
    let t95927 = t26563 * t2471;
    let t95930 = F::cast_from(0.46263278077393568556e-2_f64) * t26576 * t10985;
    (t95911, t95914, t95925, t95927, t95930)
}
