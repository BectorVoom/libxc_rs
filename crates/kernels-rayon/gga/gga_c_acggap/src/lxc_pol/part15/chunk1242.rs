//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1242/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1242(t31160: f64, t35373: f64, t37519: f64, t37522: f64, t37523: f64, t37524: f64, t37525: f64, t37526: f64, t39907: f64, t39910: f64, t39914: f64, t39919: f64, t39923: f64, t39925: f64, t39928: f64, t39930: f64, t39932: f64, t39934: f64) -> f64 {
    let t41856 = 0.4584375e-1_f64 * t39907 + 0.305625e-1_f64 * t39910 - 0.34299214494455789578e-2_f64 * t31160 - 0.85748036236139473944e-3_f64 * t39914 - t35373 + 0.64311027177104605458e-2_f64 * t39919 - 0.6431102717710460546e-2_f64 * t39923 - t37519 - 11.0_f64 / 288.0_f64 * t39925 + t37522 + t37523 + t37524 - t37525 + 0.4584375e-1_f64 * t39928 + 0.13719685797782315831e-1_f64 * t39930 - t37526 - 0.68598428988911579156e-2_f64 * t39932 - 0.68598428988911579156e-2_f64 * t39934;
    t41856
}
