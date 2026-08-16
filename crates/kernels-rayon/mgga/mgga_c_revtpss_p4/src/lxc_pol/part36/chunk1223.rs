//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1223/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1223(t25410: f64, t93189: f64, t25374: f64, t93169: f64, t2453: f64, t555: f64, t25898: f64, t25304: f64, t25876: f64, t25931: f64, t25894: f64, t25945: f64, t9285: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93371 = t93189 * t25410;
    let t93377 = t93169 * t25374;
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    let t94390 = t25304 * t555;
    let t94391 = t94390 * t25898;
    let t94394 = t25876 * t25931;
    let t94395 = t25894 * t94394;
    let t94407 = t25945 * t9285;
    (t93371, t93377, t94382, t94383, t94390, t94391, t94394, t94395, t94407)
}
