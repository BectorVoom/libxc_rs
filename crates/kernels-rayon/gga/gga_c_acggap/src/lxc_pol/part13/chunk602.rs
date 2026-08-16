//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 602/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk602(t384: f64, t4503: f64, t1150: f64, t3215: f64, t3218: f64, t3229: f64, t3231: f64, t3233: f64, t3235: f64, t3238: f64, t3240: f64, t3246: f64, t3271: f64, t3273: f64, t3280: f64, t3293: f64, t335: f64, t3616: f64, t367: f64, t4480: f64, t4484: f64, t4488: f64, t4492: f64, t4494: f64, t4496: f64) -> f64 {
    let t4505 = 0.85748036236139473944e-3_f64 * t384 * t4503;
    let t4507 = -t3215 - t3218 - 0.17149607247227894789e-2_f64 * t3229 + 0.85748036236139473944e-3_f64 * t3231 - 0.85748036236139473944e-3_f64 * t3233 + 0.40015750243531754508e-2_f64 * t3235 - 0.80031500487063509016e-2_f64 * t3238 + 0.80031500487063509016e-2_f64 * t3240 - t3246 + t367 * t4480 / 48.0_f64 + t1150 * t4484 / 16.0_f64 - t3616 * t4488 / 4.0_f64 - t4492 - t4494 + t335 * t4496 / 48.0_f64 + 0.42874018118069736972e-3_f64 * t3271 - 0.85748036236139473944e-3_f64 * t3273 - 0.20007875121765877254e-2_f64 * t3280 - t4505 - 0.12862205435420921092e-2_f64 * t3293;
    t4507
}
