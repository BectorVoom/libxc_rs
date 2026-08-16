//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 829/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk829(t7575: f64, t9653: f64, t7463: f64, t7516: f64, t9266: f64, t9267: f64, t9269: f64, t9277: f64, t9278: f64, t9609: f64, t9611: f64, t9615: f64, t9619: f64, t9623: f64, t9627: f64, t9631: f64, t9634: f64, t9638: f64, t9642: f64, t9646: f64, t9650: f64) -> f64 {
    let t9654 = t7575 * t9653;
    let t9656 = t9266 - t9267 - t9269 + 0.18868855373762491241e-2_f64 * t9609 + 0.34299214494455789578e-2_f64 * t9611 - t7463 + t9615 / 32.0_f64 + t9619 / 192.0_f64 - t9623 / 128.0_f64 - t9627 / 384.0_f64 - 0.38203125e-2_f64 * t9631 - 0.21437009059034868486e-3_f64 * t9634 - 0.10718504529517434243e-3_f64 * t9638 + 0.15724046144802076034e-3_f64 * t9642 - 0.31448092289604152068e-3_f64 * t9646 + 0.21437009059034868486e-3_f64 * t9650 - 0.47172138434406228102e-2_f64 * t9654 + t9277 + t9278 + t7516;
    t9656
}
