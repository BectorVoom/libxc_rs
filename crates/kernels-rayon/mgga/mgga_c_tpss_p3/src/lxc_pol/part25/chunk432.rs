//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 432/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk432(t1535: f64, t434: f64, t1075: f64, t1078: f64, t1503: f64, t1510: f64, t1513: f64, t1516: f64) -> (f64, f64) {
    let t1536 = t1535 * t434;
    let t1542 = 0.258925e1_f64 * t1510 - t1075 + 0.301925e0_f64 * t1503 + 0.16504875e0_f64 * t1513 - t1078 + 0.82785e-1_f64 * t1516;
    (t1536, t1542)
}
