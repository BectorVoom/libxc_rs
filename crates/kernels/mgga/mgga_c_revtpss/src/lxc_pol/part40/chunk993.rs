//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 993/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk993<F: Float>(t2760: F, t72: F, t686: F, t874: F, t251: F, t9646: F, t22: F, t780: F, t2455: F, t9285: F, t2454: F, t2829: F, t779: F, t689: F, t2444: F, t887: F) -> (F, F, F, F, F) {
    let t10972 = t2760 * t72;
    let t10974 = t874 * t10972 * t686;
    let t10981 = t9646 * t251;
    let t10982 = t780 * t22;
    let t10984 = 0.19637199382202157274e-3 * t10981 * t10982;
    let t10985 = t2455 * t9285;
    let t10987 = 0.46263278077393568556e-2 * t2454 * t10985;
    let t10988 = t779 * t2829;
    let t10989 = t689 * t10988;
    let t10991 = t2444 * t887;
    (t10974, t10984, t10987, t10989, t10991)
}
