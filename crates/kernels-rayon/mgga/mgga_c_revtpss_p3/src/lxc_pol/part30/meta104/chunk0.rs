//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 641/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk641(t2398: f64, t707: f64, t150: f64, t2389: f64, t190: f64, t198: f64, t206: f64) -> (f64, f64, f64, f64) {
    let t2400 = 8.0_f64 * t2398 * t707;
    let t2401 = t150 * t2389;
    let t2402 = t2401 * t190;
    let t2403 = t198 * t206;
    (t2400, t2401, t2402, t2403)
}
