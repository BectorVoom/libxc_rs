//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2007/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2007(t25894: f64, t94394: f64, t25945: f64, t9285: f64, t25944: f64, t2482: f64, t7262: f64, t814: f64, t820: f64, t844: f64, t596: f64, t7269: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94395 = t25894 * t94394;
    let t94407 = t25945 * t9285;
    let t94409 = 0.68540937416128198417e-2_f64 * t25944 * t94407;
    let t94423 = t2482 * t7262 * t814;
    let t94429 = t820 * t7262 * t844;
    let t94443 = t2482 * t7269 * t596;
    (t94395, t94407, t94409, t94423, t94429, t94443)
}
