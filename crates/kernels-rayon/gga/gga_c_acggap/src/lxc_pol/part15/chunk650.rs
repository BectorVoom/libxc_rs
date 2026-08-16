//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 650/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk650(t1552: f64, t6394: f64, t1181: f64, t1532: f64, t5616: f64, t1759: f64, t322: f64, t1165: f64, t1163: f64, t1150: f64, t1180: f64, t3616: f64, t3816: f64, t5253: f64, t5263: f64, t5288: f64, t6376: f64, t6380: f64, t6384: f64, t6389: f64) -> (f64, f64, f64, f64) {
    let t6395 = t1552 * t6394;
    let t6396 = t1181 * t6395;
    let t6399 = t1532 * t5616;
    let t6400 = t1181 * t6399;
    let t6403 = t1759 * t322;
    let t6405 = t1165 * t1552 * t6403;
    let t6406 = t1163 * t6405;
    let t6408 = -t3616 * t6376 / 4.0_f64 - t1150 * t6380 / 16.0_f64 + t1150 * t6384 / 8.0_f64 + t1150 * t6389 / 16.0_f64 + 35.0_f64 / 432.0_f64 * t3816 + t5253 - 0.16006300097412701803e-1_f64 * t5263 + 0.17149607247227894789e-2_f64 * t1180 * t6396 - 0.85748036236139473944e-3_f64 * t1180 * t6400 - 0.85748036236139473944e-3_f64 * t6406 - t5288;
    (t6396, t6400, t6405, t6408)
}
