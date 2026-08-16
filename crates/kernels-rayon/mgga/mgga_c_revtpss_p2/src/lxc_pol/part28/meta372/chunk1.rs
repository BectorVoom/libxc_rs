//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1405/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1405(t30: f64, t33: f64, t13302: f64, t606: f64, t2258: f64, t4201: f64, t580: f64, t9342: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t13303 = t13302 * t606;
    let t13306 = t4201 * t2258;
    let t13309 = 2.0_f64 * t580;
    let t13310 = 6.0_f64 * t9342;
    let t13312 = piecewise5(t31, 0.0_f64, t34, 0.0_f64, t13309 - t13310);
    (t13303, t13306, t13312)
}
