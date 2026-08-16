//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1190/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1190(t25299: f64, t92868: f64, t7059: f64, t9288: f64, t7064: f64, t25305: f64, t7036: f64, t820: f64, t844: f64, t2482: f64, t814: f64, t228: f64, t25273: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92870 = 0.68540937416128198417e-2_f64 * t25299 * t92868;
    let t92871 = t7059 * t9288;
    let t92873 = 0.39982213492741449076e-1_f64 * t7064 * t92871;
    let t92875 = 0.91399340044406952588e-2_f64 * t25305 * t92868;
    let t92951 = t820 * t7036 * t844;
    let t92955 = t2482 * t7036 * t814;
    let t92968 = t25273 * t228;
    (t92870, t92871, t92873, t92875, t92951, t92955, t92968)
}
