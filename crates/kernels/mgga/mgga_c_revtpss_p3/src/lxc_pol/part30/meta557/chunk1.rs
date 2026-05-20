//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1998/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1998<F: Float>(t25299: F, t92894: F, t212: F, t25286: F, t689: F, t780: F, t10073: F, t1958: F, t25390: F, t886: F, t1955: F, t25308: F, t2769: F) -> (F, F, F, F) {
    let t92895 = t25299 * t92894;
    let t92901 = t689 * t212 * t25286 * t780;
    let t92905 = t10073 * t25390 * t1958 * t886;
    let t92917 = t1955 * t25308 * t2769;
    (t92895, t92901, t92905, t92917)
}
