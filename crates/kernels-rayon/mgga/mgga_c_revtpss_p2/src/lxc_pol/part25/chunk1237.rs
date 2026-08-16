//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1237/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1237(t10761: f64, t93015: f64, t92979: f64, t92982: f64, t92984: f64, t92989: f64, t92991: f64, t92996: f64, t92998: f64, t93000: f64, t93001: f64, t93004: f64, t93008: f64, t93010: f64, t93013: f64) -> f64 {
    let t93016 = t93015 * t10761;
    let t93018 = -7.0_f64 / 16.0_f64 * t92979 - t92982 / 4.0_f64 + 3.0_f64 / 16.0_f64 * t92984 - t92989 + 0.60984003371142393869e-4_f64 * t92991 - t92996 - t92998 + t93000 - 0.18292914397043087774e-2_f64 * t93001 + 0.17149607247227894789e-3_f64 * t93004 + t93008 - 0.85748036236139473943e-3_f64 * t93010 - t93013 - 0.27107389498472794076e-4_f64 * t93016;
    t93018
}
