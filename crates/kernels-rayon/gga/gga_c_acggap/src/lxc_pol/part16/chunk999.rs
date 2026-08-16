//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 999/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk999(t35315: f64, t4987: f64, t7647: f64, t1980: f64, t34487: f64, t7476: f64, t2314: f64, t31258: f64, t1982: f64, t568: f64, t13299: f64, t31057: f64, t35288: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35316 = 0.64311027177104605458e-2_f64 * t35315;
    let t35317 = t7647 * t4987;
    let t35318 = 0.17149607247227894789e-2_f64 * t35317;
    let t35348 = t1980 * t7476 * t34487;
    let t35349 = 0.7145669686344956162e-3_f64 * t35348;
    let t35359 = t31258 * t2314;
    let t35364 = t568 * t1982;
    let t35379 = t31057 * t13299 * t35288;
    (t35316, t35318, t35349, t35359, t35364, t35379)
}
