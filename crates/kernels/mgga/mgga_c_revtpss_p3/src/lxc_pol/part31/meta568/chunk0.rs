//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1980/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1980<F: Float>(t7059: F, t9288: F, t7064: F, t25305: F, t92868: F, t136: F, t2457: F, t7082: F, t25299: F, t10073: F, t1958: F, t25390: F, t886: F) -> (F, F, F, F, F, F) {
    let t92871 = t7059 * t9288;
    let t92873 = F::cast_from(0.39982213492741449076e-1_f64) * t7064 * t92871;
    let t92875 = F::cast_from(0.91399340044406952588e-2_f64) * t25305 * t92868;
    let t92894 = t7082 * t136 * t2457;
    let t92895 = t25299 * t92894;
    let t92905 = t10073 * t25390 * t1958 * t886;
    (t92871, t92873, t92875, t92894, t92895, t92905)
}
