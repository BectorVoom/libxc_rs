//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1339/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1339(t1144: f64, t13930: f64, t14107: f64, t29775: f64, t335: f64, t338: f64, t4002: f64, t51592: f64, t51599: f64, t51604: f64, t54541: f64, t54545: f64, t54550: f64, t54561: f64, t54564: f64, t54567: f64, t54572: f64, t54575: f64, t54581: f64, t6793: f64, t8616: f64, t8793: f64) -> f64 {
    let t54583 = t54541 / 1536.0_f64 + t6793 * t54545 / 24.0_f64 + t6793 * t54550 / 24.0_f64 + t29775 * t13930 / 24.0_f64 + t8793 * t51592 / 24.0_f64 + t8793 * t51599 / 24.0_f64 + t8793 * t51604 / 48.0_f64 + t54561 / 96.0_f64 - t54564 / 96.0_f64 + t54567 - t335 * t338 * t1144 * t14107 / 96.0_f64 + t54572 / 48.0_f64 - t54575 / 48.0_f64 - t8616 * t4002 / 96.0_f64 - t54581 / 32.0_f64;
    t54583
}
