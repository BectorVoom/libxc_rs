//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1193/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1193(t13829: f64, t193: f64, t524: f64, t1: f64, t46873: f64, t544: f64, t1424: f64, t42026: f64, t42029: f64, t42030: f64, t42032: f64, t48011: f64, t48013: f64, t48017: f64, t48020: f64, t48023: f64, t48026: f64) -> f64 {
    let t48029 = 0.35750489951850426669e0_f64 * t524 * t13829 * t193;
    let t48032 = t544 * t46873 * t1;
    let t48034 = 0.39722766613167140743e-1_f64 * t48032 * t1424;
    let t48037 = t48011 + t48013 + t48017 - t48020 + t48023 - t48026 + t48029 - 0.14896037479937677779e-1_f64 * t42026 - t48034 + t42029 + 0.35750489951850426669e0_f64 * t42030 + 0.35750489951850426669e0_f64 * t42032;
    t48037
}
