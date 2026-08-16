//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2156/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2156(t19907: f64, t7111: f64, t100327: f64, t100329: f64, t100332: f64, t100334: f64, t100336: f64, t100342: f64, t100343: f64, t18909: f64, t18926: f64, t18930: f64, t27526: f64, t27527: f64, t27531: f64) -> f64 {
    let t107154 = t7111 * t19907;
    let t107159 = t27526 * t27527 * t18926 / 48.0_f64 - t27526 * t27527 * t18930 / 72.0_f64 - t27526 * t27531 * t18909 / 36.0_f64 + t107154 / 864.0_f64 + 0.30488190661738479625e-2_f64 * t100327 + 0.19055119163586549765e-3_f64 * t100329 - t100332 - t100334 - t100336 - t100342 - 0.1270341277572436651e-3_f64 * t100343;
    t107159
}
