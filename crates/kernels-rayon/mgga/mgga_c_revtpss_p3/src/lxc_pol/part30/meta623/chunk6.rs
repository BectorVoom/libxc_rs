//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2149/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2149(t1565: f64, t93066: f64, t25222: f64, t4345: f64, t4349: f64, t93072: f64, t14910: f64, t25270: f64, t14678: f64, t14673: f64, t92955: f64, t14688: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99009 = t93066 * t1565;
    let t99011 = t25222 * t4345;
    let t99012 = 0.16006300097412701803e-1_f64 * t99011;
    let t99013 = t93072 * t4349;
    let t99015 = t25270 * t14910;
    let t99017 = t25270 * t14678;
    let t99019 = t92955 * t14673;
    let t99020 = 0.2032800112371413129e-3_f64 * t99019;
    let t99021 = t92955 * t14688;
    (t99009, t99012, t99013, t99015, t99017, t99020, t99021)
}
