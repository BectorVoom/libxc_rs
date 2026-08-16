//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1264/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1264(t36289: f64, t36294: f64, t37934: f64, t37937: f64, t37938: f64, t37941: f64, t37944: f64, t37945: f64, t40465: f64, t40467: f64, t40469: f64, t40472: f64, t40474: f64, t40477: f64, t40481: f64, t40485: f64, t40487: f64) -> f64 {
    let t42132 = -0.25158473831683321656e-2_f64 * t40465 + 0.34299214494455789578e-2_f64 * t40467 + 0.34299214494455789578e-2_f64 * t40469 + t37934 + t37937 - t37938 - 0.75475421495049964965e-2_f64 * t36289 + t37941 - 0.55907719625962937011e-2_f64 * t36294 + t37944 + t37945 + 0.34299214494455789578e-2_f64 * t40472 + 0.85748036236139473944e-3_f64 * t40474 + t40477 / 8.0_f64 + 0.18868855373762491242e-1_f64 * t40481 - 0.75475421495049964966e-2_f64 * t40485 + 0.42874018118069736972e-2_f64 * t40487;
    t42132
}
