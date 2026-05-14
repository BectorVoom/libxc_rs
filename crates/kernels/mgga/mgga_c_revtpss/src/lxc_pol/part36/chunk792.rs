//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 792/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk792<F: Float>(t251: F, t9646: F, t22: F, t780: F, t2455: F, t9285: F, t2454: F, t252: F, t2769: F, t786: F, t866: F, t225: F, t788: F, t9288: F, t787: F, t781: F, t9292: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10981 = t9646 * t251;
    let t10982 = t780 * t22;
    let t10984 = 0.19637199382202157274e-3 * t10981 * t10982;
    let t10985 = t2455 * t9285;
    let t10987 = 0.46263278077393568556e-2 * t2454 * t10985;
    let t10994 = t252 * t2769;
    let t10995 = t786 * t10994;
    let t11006 = t866 * t866;
    let t11007 = 1.0 / t11006;
    let t11008 = t225 * t11007;
    let t11015 = t788 * t9288;
    let t11017 = 0.30356481678079769392e-1 * t787 * t11015;
    let t11040 = 0.17073386770573548589e-1 * t9292 * t781;
    (t10982, t10984, t10985, t10987, t10995, t11006, t11007, t11008, t11015, t11017, t11040)
}
