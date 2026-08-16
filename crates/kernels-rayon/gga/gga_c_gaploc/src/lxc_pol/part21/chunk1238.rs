//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1238/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1238(t28160: f64, t11054: f64, t5640: f64, t24886: f64, t2660: f64, t10909: f64, t7416: f64, t25193: f64, t959: f64, t7482: f64, t8793: f64, t32356: f64, t723: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32936 = 0.15976219147466979032e-1_f64 * t28160;
    let t32937 = t5640 * t11054;
    let t32938 = 0.1533717038156829987e1_f64 * t32937;
    let t32940 = 0.21450293971110256002e1_f64 * t24886 * t2660;
    let t32942 = 0.87421871174939309262e2_f64 * t7416 * t10909;
    let t32943 = t25193 * t959;
    let t32944 = 0.14896037479937677779e-1_f64 * t32943;
    let t32946 = 0.14300195980740170668e1_f64 * t8793 * t7482;
    let t32948 = t32356 * t723;
    (t32936, t32938, t32940, t32942, t32944, t32946, t32948)
}
