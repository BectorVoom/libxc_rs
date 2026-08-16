//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1076/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1076(t1238: f64, t1252: f64, t1761: f64, t3487: f64, t3593: f64, t4941: f64, t4943: f64, t4945: f64, t4947: f64, t498: f64, t5053: f64, t5055: f64, t5060: f64, t5089: f64) -> f64 {
    let t5091 = 2.0_f64 * t1238 * t5060 - t1238 * t5089 - t1252 * t4945 - t1252 * t5055 - t1761 * t3487 - t1761 * t3593 + t4941 * t498 + t4943 * t498 + t4947 * t498 + t498 * t5053;
    t5091
}
