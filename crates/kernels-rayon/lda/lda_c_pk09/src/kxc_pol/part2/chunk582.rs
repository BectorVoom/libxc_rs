//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 582/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk582(t115: f64, t4281: f64, t3397: f64, t3409: f64, t3332: f64, t3339: f64, t3330: f64, t3444: f64, t3453: f64, t1039: f64, t133: f64, t131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4283 = 5.0_f64 / 27.0_f64 * t115 * t4281;
    let t4299 = 0.510767601706895_f64 * t3397;
    let t4302 = 2.2984542076810275_f64 * t3409;
    let t4303 = 0.20376679178011928_f64 * t3332;
    let t4304 = 0.033961131963353215_f64 * t3339;
    let t4313 = 0.15282509383508946_f64 * t3330;
    let t4320 = 2.2984542076810275_f64 * t3444;
    let t4322 = 6.12921122048274_f64 * t3453;
    let t4334 = t133 * t1039;
    let t4335 = t131 * t4334;
    (t4283, t4299, t4302, t4303, t4304, t4313, t4320, t4322, t4335)
}
