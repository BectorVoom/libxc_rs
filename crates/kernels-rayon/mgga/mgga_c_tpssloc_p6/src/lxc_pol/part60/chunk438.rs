//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 438/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk438(t2218: f64, t2220: f64, t2222: f64, t2224: f64, t2226: f64, t2228: f64, t2232: f64, t1437: f64, t1409: f64) -> (f64, f64, f64) {
    let t5385 = t2218 + t2220 + t2222 + t2224 + t2226 + t2228 + t2232;
    let t5389 = t1437 * t1437;
    let t5392 = t1409 * t1409;
    (t5385, t5389, t5392)
}
