//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2009/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2009(t94385: f64, t94386: f64, t94383: f64, t25304: f64, t555: f64, t25898: f64, t25876: f64, t25931: f64, t25894: f64, t25945: f64, t9285: f64, t25944: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t94387 = t94385 * t94386;
    let t94388 = t94383 * t94387;
    let t94390 = t25304 * t555;
    let t94391 = t94390 * t25898;
    let t94392 = t94391 * t94387;
    let t94394 = t25876 * t25931;
    let t94395 = t25894 * t94394;
    let t94407 = t25945 * t9285;
    let t94409 = 0.68540937416128198417e-2_f64 * t25944 * t94407;
    (t94388, t94390, t94391, t94392, t94394, t94395, t94407, t94409)
}
