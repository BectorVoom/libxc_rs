//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 472/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk472<F: Float>(t1121: F, t471: F, t126: F, t1263: F, t371: F, t482: F, t676: F, t481: F, t225: F, t3566: F) -> (F, F, F, F, F) {
    let t3628 = t471 * t1121;
    let t3634 = t126 * t1263;
    let t3655 = t371 * t676 * t482;
    let t3657 = F::cast_from(0.47637797908966374413e-4_f64) * t481 * t3655;
    let t3670 = t3566 * t225;
    (t3628, t3634, t3655, t3657, t3670)
}
