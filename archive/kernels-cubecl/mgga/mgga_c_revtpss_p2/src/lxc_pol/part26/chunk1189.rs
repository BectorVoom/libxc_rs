//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1189/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1189<F: Float>(t2435: F, t26560: F, t10073: F, t2066: F, t25390: F, t886: F, t7058: F, t95730: F, t10665: F, t2061: F, t2439: F, t26434: F, t887: F) -> (F, F, F, F, F) {
    let t95905 = t2435 * t26560;
    let t95911 = t10073 * t25390 * t2066 * t886;
    let t95914 = F::cast_from(0.22487184191643109717e-1_f64) * t7058 * t95730;
    let t95915 = t2061 * t10665;
    let t95925 = t2439 * t26434 * t887;
    (t95905, t95911, t95914, t95915, t95925)
}
