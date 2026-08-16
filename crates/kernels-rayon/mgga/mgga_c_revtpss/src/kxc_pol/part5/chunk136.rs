//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 136/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk136(t342: f64, t386: f64, t198: f64, t293: f64, t328: f64, t330: f64, t336: f64, t265: f64) -> (f64, f64, f64) {
    let t389 = 1.0_f64 + 0.65854491829355115987e0_f64 * t342 * t386;
    let t390 = f64::ln(t389);
    let t393 = t198 * t336 * t390 - t293 + t328 + t330;
    let t394 = t265 < t393;
    let t395 = piecewise3(t394, t393, t265);
    (t389, t395, t393)
}
