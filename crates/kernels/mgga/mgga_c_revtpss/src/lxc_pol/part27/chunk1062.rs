//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1062/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1062<F: Float>(t25300: F, t9285: F, t25299: F, t7059: F, t9288: F, t7064: F, t25305: F, t25292: F, t25322: F, t25326: F, t25344: F, t25383: F, t25391: F, t25394: F, t2772: F, t92841: F, t92844: F, t92847: F, t92856: F, t92858: F, t92861: F, t92864: F) -> (F, F) {
    let t92868 = t25300 * t9285;
    let t92870 = 0.68540937416128198417e-2 * t25299 * t92868;
    let t92871 = t7059 * t9288;
    let t92873 = 0.39982213492741449076e-1 * t7064 * t92871;
    let t92875 = 0.91399340044406952588e-2 * t25305 * t92868;
    let t92876 = -0.15421710918628844643e0 * t92841 + 0.86736281882051994623e-1 * t92844 + 0.29272321618148349057e-1 * t92847 + 0.26020884564615598386e1 * t25383 * t25344 + 0.26020884564615598386e1 * t25383 * t25326 + 0.52041769129231196772e1 * t25383 * t25292 + 0.16463622957338778996e-1 * t92856 - 0.21951497276451705329e-1 * t92858 + t92861 + 0.39512695097613069591e1 * t25322 * t2772 - 0.52041769129231196772e1 * t25391 * t92864 * t25394 - t92870 - t92873 + t92875;
    (t92871, t92876)
}
