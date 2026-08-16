//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1358/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1358(t62690: f64, t69945: f64, t69948: f64, t69950: f64, t69952: f64, t69954: f64, t69956: f64, t69958: f64, t69960: f64, t69962: f64, t69964: f64, t69966: f64, t69968: f64) -> f64 {
    let t72057 = -t69945 / 2.0_f64 + t69948 / 4.0_f64 - t62690 - t69950 / 192.0_f64 + 7.0_f64 / 288.0_f64 * t69952 - 35.0_f64 / 288.0_f64 * t69954 - t69956 / 384.0_f64 - t69958 / 768.0_f64 - 5.0_f64 / 192.0_f64 * t69960 - t69962 / 128.0_f64 + t69964 / 128.0_f64 + t69966 / 192.0_f64 - t69968 / 768.0_f64;
    t72057
}
