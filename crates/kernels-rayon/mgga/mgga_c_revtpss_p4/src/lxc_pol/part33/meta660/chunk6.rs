//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2143/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2143(t18435: f64, t27159: f64, t29598: f64, t890: f64, t27383: f64, t18838: f64, t30: f64, t2411: f64, t29704: f64, t18875: f64, t98658: f64, t92790: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t106498 = t27159 * t18435;
    let t106501 = t29598 * t890;
    let t106502 = t27383 * t106501;
    let t106510 = t30 * t18838;
    let t106516 = t29704 * t2411;
    let t106520 = t98658 * t18875;
    let t106528 = t92790 * t29598;
    (t106498, t106501, t106502, t106510, t106516, t106520, t106528)
}
