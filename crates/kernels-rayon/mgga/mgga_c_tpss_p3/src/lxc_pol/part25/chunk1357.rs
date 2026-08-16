//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1357/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1357(t66390: f64, t66393: f64, t66394: f64, t66399: f64, t69926: f64, t69928: f64, t69930: f64, t69932: f64, t69934: f64, t69936: f64, t69938: f64, t69941: f64) -> f64 {
    let t72044 = -5.0_f64 / 96.0_f64 * t69926 + t69928 / 96.0_f64 - t69930 / 48.0_f64 - t66390 + t69932 / 192.0_f64 + t69934 / 192.0_f64 - 7.0_f64 / 144.0_f64 * t69936 + t69938 / 384.0_f64 - t66393 - t66394 + t66399 + t69941 / 8.0_f64;
    t72044
}
