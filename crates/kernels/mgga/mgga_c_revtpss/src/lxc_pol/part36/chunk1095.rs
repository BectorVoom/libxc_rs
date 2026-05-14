//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1095/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1095<F: Float>(t25386: F, t92837: F, t25372: F, t11015: F, t7018: F, t25300: F, t9285: F, t25299: F, t7059: F, t9288: F, t7064: F, t25305: F, t7036: F, t820: F, t844: F, t2482: F, t814: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92838 = t25386 * t92837;
    let t92843 = t25372 * t92837;
    let t92861 = 0.30356481678079769392e-1 * t7018 * t11015;
    let t92868 = t25300 * t9285;
    let t92870 = 0.68540937416128198417e-2 * t25299 * t92868;
    let t92871 = t7059 * t9288;
    let t92873 = 0.39982213492741449076e-1 * t7064 * t92871;
    let t92875 = 0.91399340044406952588e-2 * t25305 * t92868;
    let t92951 = t820 * t7036 * t844;
    let t92955 = t2482 * t7036 * t814;
    (t92838, t92843, t92861, t92870, t92871, t92873, t92875, t92951, t92955)
}
