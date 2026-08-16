//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2021/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2021(t213: f64, t25286: f64, t251: f64, t25304: f64, t25374: f64, t10505: f64, t93172: f64, t2453: f64, t25398: f64, t10506: f64, t10982: f64, t1949: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93186 = t213 * t25286;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    let t93191 = t93172 * t10505;
    let t93192 = t93190 * t93191;
    let t93194 = t2453 * t25398;
    let t93195 = t93194 * t10506;
    let t93206 = 0.19637199382202157274e-3_f64 * t9646 * t1949 * t10982;
    (t93186, t93189, t93190, t93191, t93192, t93194, t93195, t93206)
}
