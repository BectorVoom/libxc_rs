//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1205/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1205(t102409: f64, t102411: f64, t102434: f64, t102439: f64, t102462: f64, t102582: f64, t102656: f64, t114636: f64, t114660: f64, t115166: f64, t25930: f64, t26079: f64, t26304: f64, t27868: f64, t28911: f64, t30105: f64, t30252: f64, t4003: f64, t7295: f64, t86413: f64, t86641: f64, t96473: f64, t96491: f64, t97933: f64) -> f64 {
    let t115238 = -0.26020884564615598386e1_f64 * t7295 * t26079 * t115166 * t4003 - 0.28912093960683998208e-1_f64 * t102409 + 0.51405703062096148812e-1_f64 * t102411 + 0.39029762157531132076e-1_f64 * t102434 - 0.34697458558045176417e-2_f64 * t102439 - 0.52041769129231196772e1_f64 * t97933 * t30252 + 0.21951497276451705329e-1_f64 * t102462 + 0.19514881078765566038e-2_f64 * t102582 - 0.26020884564615598386e1_f64 * t25930 * t26304 * t114636 - 0.26020884564615598386e1_f64 * t27868 * t28911 * t86413 + 0.52041769129231196772e1_f64 * t25930 * t28911 * t114660 - t96473 - 0.52041769129231196772e1_f64 * t25930 * t102656 * t30105 + t96491 + 0.13010442282307799193e1_f64 * t27868 * t26304 * t86641;
    t115238
}
